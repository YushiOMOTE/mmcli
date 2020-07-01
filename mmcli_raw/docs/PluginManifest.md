# PluginManifest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Globally unique identifier that represents the plugin. | [optional]
**name** | Option<**String**> | Name of the plugin. | [optional]
**description** | Option<**String**> | Description of what the plugin is and does. | [optional]
**version** | Option<**String**> | Version number of the plugin. | [optional]
**min_server_version** | Option<**String**> | The minimum Mattermost server version required for the plugin.  Available as server version 5.6.  | [optional]
**backend** | Option<[**crate::models::PluginManifestBackend**](PluginManifest_backend.md)> |  | [optional]
**server** | Option<[**crate::models::PluginManifestServer**](PluginManifest_server.md)> |  | [optional]
**webapp** | Option<[**crate::models::PluginManifestWebapp**](PluginManifest_webapp.md)> |  | [optional]
**settings_schema** | Option<[**serde_json::Value**](.md)> | Settings schema used to define the System Console UI for the plugin. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


