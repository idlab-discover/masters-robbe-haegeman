pub mod error;

use crate::error::{Error, Result};
use async_trait::async_trait;
use either::Either;

use kube::{
    Api, Client, Resource, ResourceExt,
    api::{
        ApiResource, DeleteParams, DynamicObject, GetParams, ListParams, ObjectList, Patch,
        PatchParams, PostParams, Request,
    },
    core::Status,
    runtime::{Controller, watcher},
};
use kube_core::{NamespaceResourceScope, object::HasStatus};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fmt::Debug, mem};
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    prim_res: DynamicObject,
    sec_res: Vec<DynamicObject>,
}

// https://fasterthanli.me/articles/catching-up-with-async-rust
// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
#[async_trait]
pub trait PrimaryResource: ResourceExt + HasStatus
where
    // Requirement for initialize_status which is used almost everywhere
    <Self as HasStatus>::Status: Default,
{
    fn cache_secondary_status_dependent(&self) -> Option<&Vec<DynamicObject>>;
    fn cache_secondary_mut_status_dependent(&mut self) -> Option<&mut Vec<DynamicObject>>;

    fn initialize_status(&mut self) {
        if self.status().is_none() {
            *self.status_mut() = Some(<Self as HasStatus>::Status::default())
        }
    }

    fn cache_secondary(&mut self) -> &Vec<DynamicObject> {
        if self.status().is_none() {
            debug!("Status was created");
            self.initialize_status();
        }
        self.cache_secondary_status_dependent()
            .expect("Should always exist since the status is initialized")
    }

    fn cache_secondary_mut(&mut self) -> &mut Vec<DynamicObject> {
        if self.status().is_none() {
            debug!("Status was created");
            self.initialize_status();
        }
        self.cache_secondary_mut_status_dependent()
            .expect("Should always exist since the status is initialized")
    }

    async fn get_latest_with_secondaries(&self, client: Client) -> Result<Self>
    where
        Self: DeserializeOwned,
        <Self as Resource>::DynamicType: Default,
    {
        let dyn_type = Self::DynamicType::default();
        let url_path = format!(
            "/apis/primary-all/v1/{}/{}/{}/{}",
            Self::group(&dyn_type),
            Self::version(&dyn_type),
            Self::kind(&dyn_type),
            self.namespace()
                .unwrap_or_else(|| { String::from("default") }), // Only allocate when necessary
        );
        debug!(
            "Get primary ({}) from url_path: {}",
            self.name_any(),
            url_path
        );
        // This is also how `get` is implemented in kube.rs
        let request_builder = Request::new(url_path);
        let request = request_builder.get(&self.name_any(), &GetParams::default())?;
        let mut response: Response = client.request(request).await?;

        let mut prim_res = response.prim_res.try_parse::<Self>().map_err(Error::from)?;
        // Get a mutable reference to the Option value inside cache_secondary.
        if self.status().is_none() {
            prim_res.initialize_status();
        }
        let cache = prim_res.cache_secondary_mut();
        mem::swap(&mut response.sec_res, cache);

        Ok(prim_res)
    }

    async fn get_secondary<
        K: Resource<Scope = NamespaceResourceScope> + Clone + Debug + Serialize + DeserializeOwned,
    >(
        &mut self,
        client: Client,
        name: &str,
    ) -> Result<K>
    where
        <K as Resource>::DynamicType: Default,
    {
        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let resource = secondary_api.get(name).await.map_err(Error::from)?;
        self.update_secondary_dynamic_object(&resource);

        Ok(resource)
    }

    async fn list_secondary<
        K: Resource<Scope = NamespaceResourceScope> + Clone + Debug + Serialize + DeserializeOwned,
    >(
        &mut self,
        client: Client,
        lp: &ListParams,
    ) -> Result<ObjectList<K>>
    where
        <K as Resource>::DynamicType: Default,
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
        K: Resource<Scope = NamespaceResourceScope>
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
        <Self as Resource>::DynamicType: Default,
        <K as Resource>::DynamicType: Default,
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

        self.cache_secondary_mut().push(DynamicObject::new(
            &resource.name_any(),
            &ApiResource::erase::<K>(&K::DynamicType::default()),
        ));

        info!(
            "{}: Current secondary resources vec: {:?}",
            self.name_any(),
            self.cache_secondary()
        );

        Ok(resource)
    }

    async fn delete_secondary<
        K: Resource<Scope = NamespaceResourceScope> + Clone + Debug + Serialize + DeserializeOwned,
    >(
        &mut self,
        client: Client,
        name: &str,
        dp: &DeleteParams,
    ) -> Result<Either<K, Status>>
    where
        <K as Resource>::DynamicType: Default,
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
        K: Resource<Scope = NamespaceResourceScope>
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
        <K as Resource>::DynamicType: Default,
    {
        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let resource = secondary_api
            .patch(name, pp, patch)
            .await
            .map_err(Error::KubeError)?;
        self.update_secondary_dynamic_object(&resource);

        Ok(resource)
    }

    fn update_secondary_dynamic_object<K: Resource<Scope = NamespaceResourceScope>>(
        &mut self,
        new_res: &K,
    ) where
        <K as Resource>::DynamicType: Default,
    {
        let res_dyn = DynamicObject::new(
            &new_res.name_any(),
            &ApiResource::erase::<K>(&K::DynamicType::default()),
        );

        let cached_resource = self
            .cache_secondary_mut()
            .iter_mut()
            .find(|cached_resource| cached_resource.name_any() == new_res.name_any());

        if let Some(cached_resource) = cached_resource {
            *cached_resource = res_dyn;
        } else {
            self.cache_secondary_mut().push(res_dyn);
        }
    }

    fn delete_secondary_dynamic_object<K: Resource<Scope = NamespaceResourceScope>>(
        &mut self,
        new_res: &K,
    ) -> Result<()> {
        let cached_resource_index = self
            .cache_secondary_mut()
            .iter()
            .position(|cached_resource| cached_resource.name_any() == new_res.name_any());

        if let Some(cached_resource_index) = cached_resource_index {
            self.cache_secondary_mut()
                .swap_remove(cached_resource_index);
            Ok(())
        } else {
            Err(Error::InvalidDeleteError(new_res.name_any()))
        }
    }

    // Where clause was directly taken from [owns](https://docs.rs/kube/latest/kube/runtime/struct.Controller.html#method.owns) apart from Sync, which was required by the compiler
    fn setup_watches<
        Child: Resource<Scope = NamespaceResourceScope>
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
