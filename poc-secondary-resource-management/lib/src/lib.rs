pub mod error;

use crate::error::{Error, Result};
use async_trait::async_trait;
use either::Either;

use kube::{
    Api, Client, Resource, ResourceExt,
    api::{
        ApiResource, DeleteParams, DynamicObject, ListParams, ObjectList, Patch, PatchParams,
        PostParams,
    },
    core::Status,
    runtime::{Controller, watcher},
};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

// https://fasterthanli.me/articles/catching-up-with-async-rust
// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
#[async_trait]
pub trait PrimaryResource: kube::ResourceExt {
    fn initialize_status(&mut self);
    fn cache_secondary(&self) -> Result<&Vec<DynamicObject>>;
    fn cache_secondary_mut(&mut self) -> Result<&mut Vec<DynamicObject>>;

    async fn get_secondary<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + Serialize
            + DeserializeOwned,
    >(
        &mut self,
        client: Client,
        name: &str,
    ) -> Result<K>
    where
        <K as kube::Resource>::DynamicType: std::default::Default,
    {
        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let resource = secondary_api.get(name).await.map_err(Error::KubeError)?;
        self.update_secondary_dynamic_object(&resource)?;

        Ok(resource)
    }

    async fn list_secondary<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + Serialize
            + DeserializeOwned,
    >(
        &mut self,
        client: Client,
        lp: &ListParams,
    ) -> Result<ObjectList<K>>
    where
        <K as kube::Resource>::DynamicType: std::default::Default,
    {
        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let res_list = secondary_api.list(lp).await.map_err(Error::KubeError)?;

        let _ = res_list
            .iter()
            .map(|resource| self.update_secondary_dynamic_object(resource));

        Ok(res_list)
    }

    async fn create_secondary<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + Serialize
            + DeserializeOwned
            + Send
            + Sync,
    >(
        &mut self,
        client: Client,
        pp: &mut PostParams,
        data: &mut K,
    ) -> Result<K>
    where
        <Self as kube::Resource>::DynamicType: std::default::Default,
        <K as kube::Resource>::DynamicType: std::default::Default,
    {
        let owner_ref = self
            .controller_owner_ref(&Self::DynamicType::default())
            .expect("Assured by docs that unwrapping is safe");
        data.meta_mut()
            .owner_references
            .get_or_insert(Vec::new())
            .push(owner_ref);

        data.labels_mut().insert(
            String::from("primary_resource_label"),
            self.uid().ok_or(Error::MissingUIDError(self.name_any()))?,
        );

        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let resource = secondary_api
            .create(pp, data)
            .await
            .map_err(Error::KubeError)?;

        self.cache_secondary_mut()?.push(DynamicObject::new(
            &resource.name_any(),
            &ApiResource::erase::<K>(&K::DynamicType::default()),
        ));

        log::info!(
            "{}: Current secondary resources vec: {:?}",
            self.name_any(),
            self.cache_secondary().unwrap_or(&Vec::new())
        );

        Ok(resource)
    }

    async fn delete_secondary<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + Serialize
            + DeserializeOwned,
    >(
        &mut self,
        client: Client,
        name: &str,
        dp: &DeleteParams,
    ) -> Result<Either<K, Status>>
    where
        <K as kube::Resource>::DynamicType: std::default::Default,
    {
        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let resp = secondary_api
            .delete(name, dp)
            .await
            .map_err(Error::KubeError)?;
        if let Some(resource) = resp.as_ref().left() {
            self.delete_secondary_dynamic_object(resource)?;
        } else {
            return Err(Error::InvalidDeleteError(String::from(name)));
        }

        Ok(resp)
    }

    async fn patch_secondary<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + Serialize
            + DeserializeOwned
            + Send
            + Sync,
    >(
        &mut self,
        client: Client,
        name: &str,
        pp: &PatchParams,
        patch: &Patch<K>,
    ) -> Result<K>
    where
        <K as kube::Resource>::DynamicType: std::default::Default,
    {
        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let resource = secondary_api
            .patch(name, pp, patch)
            .await
            .map_err(Error::KubeError)?;
        self.update_secondary_dynamic_object(&resource)?;

        Ok(resource)
    }

    fn update_secondary_dynamic_object<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>,
    >(
        &mut self,
        new_res: &K,
    ) -> Result<()>
    where
        <K as kube::Resource>::DynamicType: std::default::Default,
    {
        let res_dyn = DynamicObject::new(
            &new_res.name_any(),
            &ApiResource::erase::<K>(&K::DynamicType::default()),
        );

        let sec_resources = self.cache_secondary_mut()?;
        let cached_resource = sec_resources
            .iter_mut()
            .find(|cached_resource| cached_resource.name_any() == new_res.name_any());

        if let Some(cached_resource) = cached_resource {
            *cached_resource = res_dyn;
        } else {
            sec_resources.push(res_dyn);
        }

        Ok(())
    }

    fn delete_secondary_dynamic_object<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>,
    >(
        &mut self,
        new_res: &K,
    ) -> Result<()> {
        let sec_resources = self.cache_secondary_mut()?;
        let cached_resource_index = sec_resources
            .iter()
            .position(|cached_resource| cached_resource.name_any() == new_res.name_any());

        if let Some(cached_resource_index) = cached_resource_index {
            sec_resources.swap_remove(cached_resource_index);
        } else {
            return Err(Error::InvalidDeleteError(new_res.name_any()));
        }

        Ok(())
    }

    // Where clause was directly taken from [owns](https://docs.rs/kube/latest/kube/runtime/struct.Controller.html#method.owns) apart from Sync, which was required by the compiler
    fn setup_watches<
        Child: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + DeserializeOwned
            + Send
            + Resource<DynamicType = ()>
            + 'static,
    >(
        controller: Controller<Self>,
        client: Client,
        ns: Option<&str>,
    ) -> Controller<Self>
    where
        Self: Clone + Resource + DeserializeOwned + Debug + Send + Sync + 'static,
        Self::DynamicType: Eq + std::hash::Hash + Clone,
    {
        let resource_api: Api<Child> = if let Some(ns) = ns {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };
        controller.owns(resource_api, watcher::Config::default())
    }
}
