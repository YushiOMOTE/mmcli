# InlineObject56

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**terms** | **String** | The search terms as inputed by the user. To search for posts from a user include `from:someusername`, using a user's username. To search in a specific channel include `in:somechannel`, using the channel name (not the display name). | 
**is_or_search** | **bool** | Set to true if an Or search should be performed vs an And search. | 
**time_zone_offset** | Option<**i32**> | Offset from UTC of user timezone for date searches. | [optional][default to 0]
**include_deleted_channels** | Option<**bool**> | Set to true if deleted channels should be included in the search. (archived channels) | [optional]
**page** | Option<**i32**> | The page to select. (Only works with Elasticsearch) | [optional][default to 0]
**per_page** | Option<**i32**> | The number of posts per page. (Only works with Elasticsearch) | [optional][default to 60]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


