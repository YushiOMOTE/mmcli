# PostMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**embeds** | Option<[**Vec<crate::models::PostMetadataEmbeds>**](PostMetadata_embeds.md)> | Information about content embedded in the post including OpenGraph previews, image link previews, and message attachments. This field will be null if the post does not contain embedded content.  | [optional]
**emojis** | Option<[**Vec<crate::models::Emoji>**](Emoji.md)> | The custom emojis that appear in this point or have been used in reactions to this post. This field will be null if the post does not contain custom emojis.  | [optional]
**files** | Option<[**Vec<crate::models::FileInfo>**](FileInfo.md)> | The FileInfo objects for any files attached to the post. This field will be null if the post does not have any file attachments.  | [optional]
**images** | Option<[**Vec<crate::models::PostMetadataImages>**](PostMetadata_images.md)> | An object mapping the URL of an external image to an object containing the dimensions of that image. This field will be null if the post or its embedded content does not reference any external images.  | [optional]
**reactions** | Option<[**Vec<crate::models::Reaction>**](Reaction.md)> | Any reactions made to this point. This field will be null if no reactions have been made to this post.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


