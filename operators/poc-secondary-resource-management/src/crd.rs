use crate::error::{Error, Result};
use kube::{
    api::{ApiResource, DynamicObject, PostParams},
    core::object::HasStatus,
    Api, Client, ResourceExt,
};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "poc.sec.res", version = "v1", kind = "Database", namespaced)]
#[kube(status = "DatabaseStatus", shortname = "db")]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSpec {
    #[serde(rename = "CRVersion")]
    pub cr_version: String,
    pub backup_enabled: bool, // Normally part of another BackupSpec
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    // Note: these will have an OwnerReference as well, but there is no way in Kubernetes to query them
    // apart from getting all resources and filtering on the value
    #[serde(skip)]
    pub sec_recs: Vec<DynamicObject>,
}

impl PrimaryResource for Database {
    fn secondary_resources(&self) -> Result<&Vec<DynamicObject>> {
        Ok(&self
            .status()
            .ok_or(Error::MissingStatusError(self.name_any()))?
            .sec_recs)
    }

    fn secondary_resources_mut(&mut self) -> Result<&mut Vec<DynamicObject>> {
        let name = self.name_any().clone();
        if let Some(status) = self.status_mut() {
            return Ok(&mut status.sec_recs);
        }
        Err(Error::MissingStatusError(name))
    }
}

pub(crate) trait PrimaryResource: kube::ResourceExt {
    fn secondary_resources(&self) -> Result<&Vec<DynamicObject>>;
    fn secondary_resources_mut(&mut self) -> Result<&mut Vec<DynamicObject>>;

    async fn create_secondary<
        K: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope, DynamicType = K>
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
        <K as kube::Resource>::DynamicType: Default,
    {
        let secondary_api: Api<K> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        let res = secondary_api
            .create(pp, data)
            .await
            .map_err(Error::KubeError)?;

        self.secondary_resources_mut()
            .unwrap()
            .push(DynamicObject::new(
                &res.name_any(),
                &ApiResource::erase::<K>(&res),
            ));

        Ok(res)
    }
}
