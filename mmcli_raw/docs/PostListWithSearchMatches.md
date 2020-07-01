# PostListWithSearchMatches

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order** | Option<**Vec<String>**> |  | [optional]
**posts** | Option<[**::std::collections::HashMap<String, crate::models::Post>**](Post.md)> |  | [optional]
**matches** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | A mapping of post IDs to a list of matched terms within the post. This field will only be populated on servers running version 5.1 or greater with Elasticsearch enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


