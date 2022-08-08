use sea_orm :: prelude :: * ; pub fn filter_recursive (root_filter : Option < Filter >) -> sea_orm :: Condition { let mut condition = sea_orm :: Condition :: all () ; if let Some (current_filter) = root_filter { if let Some (or_filters) = current_filter . or { let or_condition = or_filters . into_iter () . fold (sea_orm :: Condition :: any () , | fold_condition , filter | fold_condition . add (filter_recursive (Some (* filter)))) ; condition = condition . add (or_condition) ; } if let Some (and_filters) = current_filter . and { let and_condition = and_filters . into_iter () . fold (sea_orm :: Condition :: all () , | fold_condition , filter | fold_condition . add (filter_recursive (Some (* filter)))) ; condition = condition . add (and_condition) ; } if let Some (media_type_id) = current_filter . media_type_id { if let Some (eq_value) = media_type_id . eq { condition = condition . add (Column :: MediaTypeId . eq (eq_value)) } if let Some (ne_value) = media_type_id . ne { condition = condition . add (Column :: MediaTypeId . ne (ne_value)) } if let Some (gt_value) = media_type_id . gt { condition = condition . add (Column :: MediaTypeId . gt (gt_value)) } if let Some (gte_value) = media_type_id . gte { condition = condition . add (Column :: MediaTypeId . gte (gte_value)) } if let Some (lt_value) = media_type_id . lt { condition = condition . add (Column :: MediaTypeId . lt (lt_value)) } if let Some (lte_value) = media_type_id . lte { condition = condition . add (Column :: MediaTypeId . lte (lte_value)) } if let Some (is_in_value) = media_type_id . is_in { condition = condition . add (Column :: MediaTypeId . is_in (is_in_value)) } if let Some (is_not_in_value) = media_type_id . is_not_in { condition = condition . add (Column :: MediaTypeId . is_not_in (is_not_in_value)) } if let Some (is_null_value) = media_type_id . is_null { if is_null_value { condition = condition . add (Column :: MediaTypeId . is_null ()) } } } if let Some (name) = current_filter . name { if let Some (eq_value) = name . eq { condition = condition . add (Column :: Name . eq (eq_value)) } if let Some (ne_value) = name . ne { condition = condition . add (Column :: Name . ne (ne_value)) } if let Some (gt_value) = name . gt { condition = condition . add (Column :: Name . gt (gt_value)) } if let Some (gte_value) = name . gte { condition = condition . add (Column :: Name . gte (gte_value)) } if let Some (lt_value) = name . lt { condition = condition . add (Column :: Name . lt (lt_value)) } if let Some (lte_value) = name . lte { condition = condition . add (Column :: Name . lte (lte_value)) } if let Some (is_in_value) = name . is_in { condition = condition . add (Column :: Name . is_in (is_in_value)) } if let Some (is_not_in_value) = name . is_not_in { condition = condition . add (Column :: Name . is_not_in (is_not_in_value)) } if let Some (is_null_value) = name . is_null { if is_null_value { condition = condition . add (Column :: Name . is_null ()) } } } } condition } pub use crate :: orm :: media_types :: * ; use crate :: graphql :: * ; # [async_graphql :: Object (name = "MediaTypes")] impl Model { pub async fn media_type_id (& self) -> & i32 { & self . media_type_id } pub async fn name (& self) -> & Option < String > { & self . name } pub async fn media_types_media_type_tracks < 'a > (& self , ctx : & async_graphql :: Context < 'a >) -> Vec < crate :: orm :: tracks :: Model > { let data_loader = ctx . data :: < async_graphql :: dataloader :: DataLoader < OrmDataloader >> () . unwrap () ; let key = MediaTypeTracksFK (self . media_type_id . clone () . try_into () . unwrap ()) ; let data : Option < _ > = data_loader . load_one (key) . await . unwrap () ; data . unwrap_or (vec ! []) } } # [derive (async_graphql :: InputObject , Debug)] # [graphql (name = "MediaTypesFilter")] pub struct Filter { pub or : Option < Vec < Box < Filter >> > , pub and : Option < Vec < Box < Filter >> > , pub media_type_id : Option < TypeFilter < i32 >> , pub name : Option < TypeFilter < String >> } # [derive (Clone , Eq , PartialEq , Hash , Debug)] pub struct MediaTypeTracksFK (i32) ; # [async_trait :: async_trait] impl async_graphql :: dataloader :: Loader < MediaTypeTracksFK > for OrmDataloader { type Value = Vec < crate :: orm :: tracks :: Model > ; type Error = std :: sync :: Arc < sea_orm :: error :: DbErr > ; async fn load (& self , keys : & [MediaTypeTracksFK]) -> Result < std :: collections :: HashMap < MediaTypeTracksFK , Self :: Value > , Self :: Error > { let filter = sea_orm :: Condition :: all () . add (sea_orm :: sea_query :: SimpleExpr :: Binary (Box :: new (sea_orm :: sea_query :: SimpleExpr :: Tuple (vec ! [sea_orm :: sea_query :: Expr :: col (crate :: orm :: tracks :: Column :: MediaTypeId . as_column_ref ()) . into_simple_expr ()])) , sea_orm :: sea_query :: BinOper :: In , Box :: new (sea_orm :: sea_query :: SimpleExpr :: Tuple (keys . iter () . map (| tuple | sea_orm :: sea_query :: SimpleExpr :: Values (vec ! [tuple . 0 . clone () . into ()])) . collect ())))) ; use itertools :: Itertools ; Ok (crate :: orm :: tracks :: Entity :: find () . filter (filter) . all (& self . db) . await ? . into_iter () . map (| model | { let key = MediaTypeTracksFK (model . media_type_id . clone () . try_into () . unwrap ()) ; (key , model) }) . into_group_map ()) } }