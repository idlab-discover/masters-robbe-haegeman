pub mod error;

use crate::error::{Error, Result};
use kube::{
    api::{ApiResource, DynamicObject, PostParams},
    Api, Client, ResourceExt,
};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait PrimaryResource: kube::ResourceExt {
    fn initialize_status(&mut self);
    fn secondary_resources(&self) -> Result<&Vec<DynamicObject>>;
    fn secondary_resources_mut(&mut self) -> Result<&mut Vec<DynamicObject>>;

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
        data: &K,
    ) -> Result<K>
    where
        <K as kube::Resource>::DynamicType: std::default::Default,
    {
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
}
