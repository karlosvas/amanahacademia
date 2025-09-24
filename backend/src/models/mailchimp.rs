use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MailchimpClient {
    pub api_key: String,
    pub list_id: String,
    pub datacenter: String,
    pub client: reqwest::Client,
}

impl MailchimpClient {
    pub fn new(api_key: String, datacenter: String, list_id: String) -> Self {
        Self {
            api_key,
            datacenter,
            list_id,
            client: reqwest::Client::new(),
        }
    }

    pub fn get_base_url(&self) -> String {
        format!("https://{}.api.mailchimp.com/3.0", self.datacenter)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub email_address: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<MergeFields>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MergeFields {
    #[serde(rename = "FNAME")]
    pub fname: Option<String>,
    #[serde(rename = "LNAME")]
    pub lname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddContactResponse {
    pub id: String,
    pub email_address: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub settings: CampaignSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignSettings {
    pub subject_line: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct ListsResponse {
    pub lists: Vec<MailchimpList>,
    pub total_items: usize,
}

#[derive(Debug, Deserialize)]
pub struct MailchimpList {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMailchimp {
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
    pub status: i32,
    pub detail: String,
    pub instance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<Member>,
    pub list_id: String,
    pub total_items: usize,
    #[serde(default)]
    pub _links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub email_address: String,
    pub unique_email_id: String,
    pub contact_id: String,
    pub full_name: String,
    pub web_id: i64,
    pub email_type: Option<String>,
    pub status: String,
    pub unsubscribe_reason: Option<String>,
    pub consents_to_one_to_one_messaging: Option<bool>,
    pub sms_phone_number: Option<String>,
    pub sms_subscription_status: Option<String>,
    pub sms_subscription_last_updated: Option<String>,
    pub merge_fields: Option<serde_json::Value>,
    pub interests: Option<serde_json::Value>,
    pub stats: Option<MemberStats>,
    pub ip_signup: Option<String>,
    pub timestamp_signup: Option<String>,
    pub ip_opt: Option<String>,
    pub timestamp_opt: Option<String>,
    pub member_rating: Option<i64>,
    pub last_changed: Option<String>,
    pub language: Option<String>,
    pub vip: Option<bool>,
    pub email_client: Option<String>,
    pub location: Option<Location>,
    pub marketing_permissions: Option<Vec<MarketingPermission>>,
    pub last_note: Option<LastNote>,
    pub source: Option<String>,
    pub tags_count: Option<i64>,
    pub tags: Option<Vec<Tag>>,
    pub list_id: String,
    #[serde(default)]
    pub _links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberStats {
    pub avg_open_rate: Option<f64>,
    pub avg_click_rate: Option<f64>,
    pub ecommerce_data: Option<EcommerceData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EcommerceData {
    pub total_revenue: Option<f64>,
    pub number_of_orders: Option<i64>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub gmtoff: Option<i64>,
    pub dstoff: Option<i64>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingPermission {
    pub marketing_permission_id: String,
    pub text: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastNote {
    pub note_id: i64,
    pub created_at: String,
    pub created_by: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub rel: String,
    pub href: String,
    pub method: String,
    pub targetSchema: Option<String>,
    pub schema: Option<String>,
}

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct MembersResponse {
//     pub members: Vec<Member>,
//     pub list_id: String,
//     pub total_items: usize,
//     #[serde(default)]
//     pub _links: Vec<Link>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Member {
//     pub id: String,
//     pub email_address: String,
//     pub unique_email_id: String,
//     pub contact_id: String,
//     pub full_name: String,
//     pub web_id: i64,
//     pub email_type: Option<String>,
//     pub status: String,
//     pub unsubscribe_reason: Option<String>,
//     pub consents_to_one_to_one_messaging: Option<bool>,
//     pub sms_phone_number: Option<String>,
//     pub sms_subscription_status: Option<String>,
//     pub sms_subscription_last_updated: Option<String>,
//     pub merge_fields: Option<serde_json::Value>,
//     pub interests: Option<serde_json::Value>,
//     pub stats: Option<MemberStats>,
//     pub ip_signup: Option<String>,
//     pub timestamp_signup: Option<String>,
//     pub ip_opt: Option<String>,
//     pub timestamp_opt: Option<String>,
//     pub member_rating: Option<i64>,
//     pub last_changed: Option<String>,
//     pub language: Option<String>,
//     pub vip: Option<bool>,
//     pub email_client: Option<String>,
//     pub location: Option<Location>,
//     pub marketing_permissions: Option<Vec<MarketingPermission>>,
//     pub last_note: Option<LastNote>,
//     pub source: Option<String>,
//     pub tags_count: Option<i64>,
//     pub tags: Option<Vec<Tag>>,
//     pub list_id: String,
//     #[serde(default)]
//     pub _links: Vec<Link>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct MemberStats {
//     pub avg_open_rate: Option<f64>,
//     pub avg_click_rate: Option<f64>,
//     pub ecommerce_data: Option<EcommerceData>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct EcommerceData {
//     pub total_revenue: Option<f64>,
//     pub number_of_orders: Option<i64>,
//     pub currency_code: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Location {
//     pub latitude: Option<f64>,
//     pub longitude: Option<f64>,
//     pub gmtoff: Option<i64>,
//     pub dstoff: Option<i64>,
//     pub country_code: Option<String>,
//     pub timezone: Option<String>,
//     pub region: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct MarketingPermission {
//     pub marketing_permission_id: String,
//     pub text: String,
//     pub enabled: bool,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LastNote {
//     pub note_id: i64,
//     pub created_at: String,
//     pub created_by: String,
//     pub note: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Tag {
//     pub id: i64,
//     pub name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Link {
//     pub rel: String,
//     pub href: String,
//     pub method: String,
//     pub targetSchema: Option<String>,
//     pub schema: Option<String>,
// }
