//! A set of functions that have been taken from the Reconcile function fo the ReconcilePerconaServerMongoDB object
//! This list isn't exhaustive and is more of a example of a possible implementation of the Reconcile function
//! The documentation above each function is a summary of the workings of the original code
//! The code used here however is a simplification, just to test the solution
use std::sync::Arc;

use k8s_openapi::api::apps;

use crate::{crd, error::Error, Context};

// Placeholder for the real struct
pub struct ReplsetSpec {
    pub name: String,
    pub size: i32,
}

/// Sets the CR version if it is not set
///
/// API requests:
/// - patch to the CR
pub async fn set_cr_version(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Checks the spec and sets the defaults if they are not set
///
/// API requests:
/// - None
///
/// The code however uses this info to make a both a get and patch request to the
pub async fn check_n_set_defaults(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Built in function from Kubebuilder
/// Kube.rs supports Add and remove functions instead
pub async fn set_finalizers(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Gets an ordered list of finalizers (through the spec) and then executes them in order
/// Is triggered when the ObjectMeta contains a DeletionTimestamp
///
/// API requests:
/// - Get request for statefulset (replset)
/// - Get request for PVC + Delete request for PVC
/// - Get request for secrets + Delete request for secrets
/// - Get request for Pods + Delete request for Pods (db query router + replset)
/// - Update request for CR
pub async fn check_finalizers(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Checks if the CR is paused and if it is (and backups are finished) it will remove the pods
///
/// API requests:
/// - Get request for backup CR
/// - Get request for statefulset (replset)
/// - Get request for Pods + Delete request for Pods (db query router + replset)
pub async fn reconcile_pause(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Checks if sharding is disabled and attempts to unshard the cluster if it is
/// Seems like an oversight, but does get requests and only then checks if it will need those objects
///
/// API requests:
/// - Get request for statefulset (cfg + database daemon)
pub async fn check_configuration(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Ensures replica set pods (which are managed through a statefulset) downscaled one by one
///
/// API requests:
/// - Get request for statefulset (replset)
pub async fn safe_downscale(_db: &crd::Database, _ctx: Arc<Context>) -> Result<bool, Error> {
    Ok(false)
}

/// Ensures the existence of the users' secret for the CR by attempting to retrieve it and creating it if it does not exist
///
/// API requests:
/// - Get request for secret
/// - Create request for secret
pub async fn reconcile_user_secret(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Retrieves the current config map, creates a new one if it does not exist, or updates the old one to the new spec (if different)
///
/// API requests:
/// - Get request for config map
/// - Create / update request for config map
pub async fn reconcile_db_config_maps(
    _db: &crd::Database,
    _ctx: Arc<Context>,
) -> Result<(), Error> {
    Ok(())
}

/// Retrieves the current config map (linked to the RS), creates a new one if it does not exist, or updates the old one to the new spec (if different)
///
/// API requests:
/// - Get request for config map
/// - Create / update request for config map
pub async fn reconcile_db_daemon_config_maps(
    _db: &crd::Database,
    _ctx: Arc<Context>,
    _repls: &mut [ReplsetSpec],
) -> Result<(), Error> {
    Ok(())
}

/// Compares the user secrets with the internal secrets and changes the internal secrets if they are different
///
/// API requests:
/// - Get request for secret (users + internal)
pub async fn reconcile_users(
    _db: &crd::Database,
    _ctx: Arc<Context>,
    _repls: &mut [ReplsetSpec],
) -> Result<(), Error> {
    Ok(())
}

/// Identifies statefulsets that need to be removed according to the spec (replset)
///
/// API requests:
/// - Get request for statefulset (replset)
pub async fn get_sts_for_removal(
    _db: &crd::Database,
    _ctx: Arc<Context>,
) -> Result<Vec<apps::v1::StatefulSet>, Error> {
    Ok(vec![])
}

/// Connects to database system, makes sure only system databases are present and errors if not
///
/// API requests:
/// - None
pub async fn check_if_possible_to_remove(
    _db: &crd::Database,
    _ctx: Arc<Context>,
    _rs_name: &str,
) -> Result<(), Error> {
    Ok(())
}

/// Attempts to remove the shard, sleeping in between each try
///
/// API requests:
/// - None
pub async fn remove_rs_from_shard(
    _db: &crd::Database,
    _ctx: Arc<Context>,
    _rs_name: &str,
) -> Result<(), Error> {
    Ok(())
}

/// Attempts to get the non-internal secret for SSL and creates it from the internal secret if it does not exist
///
/// API requests:
/// - Get request for secret (SSL)
/// - Create request for secret (SSL)
pub async fn reconcile_ssl(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Gets mongo security key or creates it if it does not exist
///
/// API requests:
/// - Get request for secret
/// - Create request for secret
pub async fn ensure_security_key(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Creates or updates cron jobs for backups and deletes them if they are old / unused
///
/// API requests:
/// - Get request for cron job
/// - Create / update request for cron job
pub async fn reconcile_backup_tasks(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Fetches all statefulsets, reconciles them individually and manages the services for the statefulsets
///
/// API requests:
/// - Get request for statefulset
/// - Get request for pods in statefulset
/// - Get request for PVCs
/// - Update request for PVCs
/// - Get request for pods
/// - Get request for events
/// - Get request for services
/// - Create / update request for services
pub async fn reconcile_repl_sets(
    _db: &crd::Database,
    _ctx: Arc<Context>,
    _repls: &mut [ReplsetSpec],
) -> Result<(), Error> {
    Ok(())
}

/// Executes several checks on the state of the query routers and deletes them if needed
///
/// API requests:
/// - Get request for statefulset
/// - Delete request for statefulset
/// - Delete request for config map
pub async fn reconcile_db_query_router(
    _db: &crd::Database,
    _ctx: Arc<Context>,
) -> Result<(), Error> {
    Ok(())
}

/// Checks if all the pods in the statefulset are up to date and sets the feature compatibility version inside the database cluster
///
/// API requests:
/// - Get request for statefulset
/// - Get request for pods
pub async fn upgrade_fcv_if_needed(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Runs in case of a downscale and deletes the PVCs that are no longer needed.
/// Loops over the all pods, stores their items and only keeps the PVC's referenced
///
/// API requests:
/// - Get request for pods
/// - Get request for PVCs
/// - Delete request for PVCs
pub async fn delete_orphan_pvcs(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Reconciles the custom users, keeping the internal database user records (through a cli binding) and the external user records in sync
///
/// API requests:
/// - Get request for secrets (internal user secret + external)
/// - Create / Update request for secrets (external user secret)
pub async fn reconcile_custom_users(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Retrieves the services for each cluster (and thus most likely CR)
///
/// API requests:
/// - Get request for service exports (a CR managed by MCS (Multi-Cluster Service))
/// - Get request for services
/// - Delete request for service exports (a CR managed by MCS (Multi-Cluster Service))
/// - Delete request for services
pub async fn export_services(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Ensures the version of the operator is up to date based on the custom resource's (CR) UpgradeOptions.Schedule
///
/// API requests:
/// - Manages cronjobs, but not sure if this is through Kubernetes or in the operator itself
/// - Get request for CR
/// - Get request for pod, replicaset and deployment of operator
pub async fn schedule_ensure_function(
    _db: &crd::Database,
    _ctx: Arc<Context>,
) -> Result<(), Error> {
    Ok(())
}

/// Ensures that the PITR (point-in-time recovery) configuration in the backup solution is up-to-date with the specifications
///
/// API requests:
/// - Get request for restore CR
pub async fn update_pitr(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}

/// Checks if the resync annotation is present and if so, removes it and resyncs the backup solution
///
/// API requests:
/// - Get request for database CR
/// - Patch request for database CR
/// - Get request for pod
pub async fn resync_backup_solution_if_needed(
    _db: &crd::Database,
    _ctx: Arc<Context>,
) -> Result<(), Error> {
    Ok(())
}

/// Updates the status of the CR
///
/// API requests:
/// - Update request for CR
pub async fn update_status(_db: &crd::Database, _ctx: Arc<Context>) -> Result<(), Error> {
    Ok(())
}
