// impl From<Visibility> for Bson {
//     fn from(visibility: Visibility) -> Self {
//         match visibility {
//             Visibility::Public => Bson::String("Public".to_string()),
//             Visibility::Private => Bson::String("Private".to_string()),
//             Visibility::Unlisted => Bson::String("Unlisted".to_string()),
//             Visibility::Locked => Bson::String("Locked".to_string()),
//         }
//     }
// }