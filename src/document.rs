use crate::ArangoError;
use serde::{Deserialize, Deserializer, Serialize};

/// Options for document insertion.
#[derive(Serialize, Deserialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentInsertOptions {
    /// Wait until document has been synced to disk.
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    #[builder(default, setter(strip_option))]
    return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result. Only available if the overwrite option is used.
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the created document.
    /// This option can be used to save some network traffic.
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
    /// If set to true, the insert becomes a replace-insert.
    /// If a document with the same _key already exists the new document is not
    /// rejected with unique constraint violated but will replace the old
    /// document.
    #[builder(default, setter(strip_option))]
    overwrite: Option<bool>,
    /// TODO add nice formatted documentation from official doc
    #[cfg(arango3_7)]
    #[builder(default, setter(strip_option))]
    overwrite_mode: Option<DocumentOverwriteMode>,
}

impl Default for DocumentInsertOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Options for document update,
#[derive(Serialize, Deserialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUpdateOptions {
    /// If the intention is to delete existing attributes with the patch
    /// command, the URL query parameter keepNull can be used with a value of
    /// false. This will modify the behavior of the patch command to remove any
    /// attributes from the existing document that are contained in the patch
    /// document with an attribute value of null.
    #[builder(default, setter(strip_option))]
    keep_null: Option<bool>,
    /// Controls whether objects (not arrays) will be merged if present in both
    /// the existing and the patch document. If set to false, the value in the
    /// patch document will overwrite the existing document’s value. If set to
    /// true, objects will be merged. The default is true.
    #[builder(default, setter(strip_option))]
    merge_objects: Option<bool>,
    /// Wait until document has been synced to disk.
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// By default, or if this is set to true, the _rev attributes in the given
    /// document is ignored. If this is set to false, then the _rev
    /// attribute given in the body document is taken as a precondition. The
    /// document is only update if the current revision is the one specified.
    #[builder(default, setter(strip_option))]
    ignore_revs: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    #[builder(default, setter(strip_option))]
    return_new: Option<bool>,
    /// Return additionally the complete previous revision of the changed
    /// document under the attribute old in the result.
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the updated document.
    /// This option can be used to save some network traffic.
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
}

impl Default for DocumentUpdateOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Serialize, Deserialize)]
pub enum DocumentOverwriteMode {
    /// If a document with the specified _key value exists already,
    /// nothing will be done and no write operation will be carried out.
    /// The insert operation will return success in this case.
    /// This mode does not support returning the old document version using
    /// RETURN OLD. When using RETURN NEW, null will be returned in case the
    /// document already existed.
    Ignore,
    /// If a document with the specified _key value exists already, it will be
    /// overwritten with the specified document value. This mode will also
    /// be used when no overwrite mode is specified but the overwrite flag is
    /// set to true.
    Replace,
    /// If a document with the specified _key value exists already, it will be
    /// patched (partially updated) with the specified document value.
    /// The overwrite mode can be further controlled via the keepNull and
    /// mergeObjects parameters
    Update,
    /// if a document with the specified _key value exists already, return a
    /// unique constraint violation error so that the insert operation fails.
    /// This is also the default behavior in case the overwrite mode is not set,
    /// and the overwrite flag is false or not set either.
    ///
    /// keepNull (optional): If the intention is to delete existing attributes
    /// with the update-insert command, the URL query parameter keepNull can be
    /// used with a value of false. This will modify the behavior of the patch
    /// command to remove any attributes from the existing document that are
    /// contained in the patch document with an attribute value of null.
    /// This option controls the update-insert behavior only.
    ///
    /// mergeObjects (optional): Controls whether objects (not arrays) will be
    /// merged if present in both the existing and the update-insert document.
    /// If set to false, the value in the patch document will overwrite the
    /// existing document’s value. If set to true, objects will be merged. The
    /// default is true. This option controls the update-insert behavior only.
    /// TODO need to implement the two extra modes keepNull & mergeObjects
    Conflict,
}

/// Options for document replace,
#[derive(Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReplaceOptions {
    /// Wait until document has been synced to disk.
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// By default, or if this is set to true, the _rev attributes in the given
    /// document is ignored. If this is set to false, then the _rev
    /// attribute given in the body document is taken as a precondition. The
    /// document is only replaced if the current revision is the one specified.
    #[builder(default, setter(strip_option))]
    ignore_revs: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    #[builder(default, setter(strip_option))]
    return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result.
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the replaced document.
    /// This option can be used to save some network traffic.
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
}

impl Default for DocumentReplaceOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Options for document reading.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DocumentReadOptions {
    /// If the “If-None-Match” header is given, then it must contain exactly one
    /// Etag. The document is returned, if it has a different revision than
    /// the given Etag. Otherwise an HTTP 304 is returned.
    IfNoneMatch(String),
    ///  If the “If-Match” header is given, then it must contain exactly one
    /// Etag. The document is returned, if it has the same revision as the
    /// given Etag. Otherwise a HTTP 412 is returned.
    IfMatch(String),
    NoHeader,
}

impl Default for DocumentReadOptions {
    fn default() -> Self {
        Self::NoHeader
    }
}

/// Options for document removes,
#[derive(Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRemoveOptions {
    /// Wait until document has been synced to disk.
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result.
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the created document.
    /// This option can be used to save some network traffic.
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
}

impl Default for DocumentRemoveOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHeader {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
}

/// Standard Response when having CRUD operation on document
/// Todo could add more response variant like shown on official doc
/// 200: is returned if the document was found
/// 304: is returned if the “If-None-Match” header is given and the document has
/// the same version 404: is returned if the document or collection was not
/// found 412: is returned if an “If-Match” header is given and the found
/// document has a different version. The response will also contain the found
/// document’s current revision in the Etag header.
pub enum DocumentResponse<T> {
    /// Silent is when there is empty object returned by the server
    Silent,
    /// Contain data after CRUD
    Response {
        header: DocumentHeader,
        old: Option<T>,
        new: Option<T>,
        _old_rev: Option<String>,
    },
    Err(ArangoError),
}

/// Gives extra method on the DocumentResponse to quickly check what the server
/// returns
impl<T> DocumentResponse<T> {
    /// Should be true when the server send back an empty object {}
    pub fn is_silent(&self) -> bool {
        match self {
            DocumentResponse::Silent => true,
            _ => false,
        }
    }
    /// Should be true if there is a response from the server
    pub fn has_response(&self) -> bool {
        match self {
            DocumentResponse::Response { .. } => true,
            _ => false,
        }
    }

    /// Return the document header contained inside the response
    pub fn header(&self) -> Option<&DocumentHeader> {
        if let DocumentResponse::Response { header, .. } = self {
            Some(header)
        } else {
            None
        }
    }
    /// Return the old document before changes
    pub fn old_doc(&self) -> Option<&T> {
        Option::from(if let DocumentResponse::Response { old, .. } = self {
            old
        } else {
            &None
        })
    }
    /// Return the new document
    pub fn new_doc(&self) -> Option<&T> {
        Option::from(if let DocumentResponse::Response { new, .. } = self {
            new
        } else {
            &None
        })
    }
    /// return the old revision of the document
    pub fn old_rev(&self) -> Option<&String> {
        Option::from(if let DocumentResponse::Response { _old_rev, .. } = self {
            _old_rev
        } else {
            &None
        })
    }
}

impl<'de, T> Deserialize<'de> for DocumentResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut obj = serde_json::Value::deserialize(deserializer)?;

        let json = obj.as_object_mut().unwrap();

        if json.contains_key("_key") != true {
            Ok(DocumentResponse::Silent)
        } else {
            let header: DocumentHeader = DocumentHeader {
                _id: serde_json::from_value(json.remove("_id").unwrap()).unwrap(),
                _key: serde_json::from_value(json.remove("_key").unwrap()).unwrap(),
                _rev: serde_json::from_value(json.remove("_rev").unwrap()).unwrap(),
            };

            let old = if json.contains_key("old") {
                T::deserialize(json.remove("old").unwrap()).ok()
            } else {
                None
            };

            let new = if json.contains_key("new") {
                T::deserialize(json.remove("new").unwrap()).ok()
            } else {
                None
            };
            let _old_rev = if json.contains_key("_old_rev") {
                Some(json.remove("_old_rev").unwrap().to_string())
            } else {
                None
            };

            Ok(DocumentResponse::Response {
                header,
                old,
                new,
                _old_rev,
            })
        }
    }
}

/// Structure that represents a document within its content and header
#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    #[serde(flatten)]
    pub header: DocumentHeader,
    #[serde(flatten)]
    pub document: T,
}

impl<'de, T> Document<T>
where
    T: Serialize + Deserialize<'de>,
{
    pub fn new(data: T) -> Self {
        Document {
            document: data,
            header: DocumentHeader {
                _id: String::new(),
                _key: String::new(),
                _rev: String::new(),
            },
        }
    }
}
