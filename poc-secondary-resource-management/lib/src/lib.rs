pub mod error;

use crate::error::{Error, Result};
use k8s_openapi::api::core;
use kube::{
    Api, Client, Resource, ResourceExt,
    api::{ApiResource, DeleteParams, DynamicObject, ListParams, Patch, PatchParams, PostParams},
    runtime::{Controller, watcher},
};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

pub trait PrimaryResource: kube::ResourceExt {
    fn initialize_status(&mut self);
    fn secondary_resources(&self) -> Result<&Vec<DynamicObject>>;
    fn secondary_resources_mut(&mut self) -> Result<&mut Vec<DynamicObject>>;

    // https://fasterthanli.me/articles/catching-up-with-async-rust
    // https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
    // Could be that we will have to transition to the async_trait crate down the line, but would introduce Send restriction
    async fn get_secondary(&self, _name: &str) {}

    async fn list_secondary(&self, _lp: &ListParams) {}

    async fn create_secondary<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + Serialize
            + DeserializeOwned,
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

        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let res = secondary_api
            .create(pp, data)
            .await
            .map_err(Error::KubeError)?;

        self.secondary_resources_mut()?.push(DynamicObject::new(
            &res.name_any(),
            &ApiResource::erase::<K>(&K::DynamicType::default()),
        ));

        log::info!(
            "{}: Current secondary resources vec: {:?}",
            self.name_any(),
            self.secondary_resources().unwrap_or(&Vec::new())
        );

        Ok(res)
    }

    async fn delete_secondary(&mut self, _name: &str, _dp: &DeleteParams) {}

    async fn patch_secondary<P: Serialize + Debug>(
        &self,
        _name: &str,
        _pp: &PatchParams,
        _patch: &Patch<P>,
    ) {
    }

    // Where clause was directly taken from [owns](https://docs.rs/kube/latest/kube/runtime/struct.Controller.html#method.owns) apart from Sync, which was required by the compiler
    fn setup_watches(
        controller: Controller<Self>,
        client: Client,
        ns: Option<&str>,
    ) -> Controller<Self>
    where
        Self: Clone + Resource<DynamicType = ()> + DeserializeOwned + Debug + Send + Sync + 'static,
    {
        let secret_api: Api<core::v1::Secret> = if let Some(ns) = ns {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };
        controller.owns(secret_api, watcher::Config::default())
    }
}
