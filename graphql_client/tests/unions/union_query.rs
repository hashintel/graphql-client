pub mod union_query { # ! [ allow ( non_camel_case_types ) ] # ! [ allow ( non_snake_case ) ] # ! [ allow ( dead_code ) ] use serde ; pub const OPERATION_NAME : & 'static str = "UnionQuery" ; pub const QUERY : & 'static str = "query UnionQuery {\n  names {\n    __typename\n    ... on Dog {\n      name\n    }\n    ... on Person {\n      firstName\n      lastName\n    }\n    ... on Organization {\n      title\n    }\n  }\n}\n\nfragment NamesFragment on NamedThing {\n  __typename\n  ... on Dog {\n    name\n  }\n  ... on Person {\n    firstName\n  }\n  ... on Organization {\n    title\n  }\n}\n\nquery FragmentOnUnion {\n  names {\n    ...NamesFragment\n  }\n}\n" ; use serde_derive :: * ; # [ allow ( dead_code ) ] type Boolean = bool ; # [ allow ( dead_code ) ] type Float = f64 ; # [ allow ( dead_code ) ] type Int = i64 ; # [ allow ( dead_code ) ] type ID = String ; # [ derive ( Deserialize ) ] pub struct UnionQueryNamesOnDog { pub name : String , } # [ derive ( Deserialize ) ] pub struct UnionQueryNamesOnOrganization { pub title : String , } # [ derive ( Deserialize ) ] pub struct UnionQueryNamesOnPerson { # [ serde ( rename = "firstName" ) ] pub first_name : String , # [ serde ( rename = "lastName" ) ] pub last_name : Option < String > , } # [ derive ( Deserialize ) ] # [ serde ( tag = "__typename" ) ] pub enum UnionQueryNames { Dog ( UnionQueryNamesOnDog ) , Organization ( UnionQueryNamesOnOrganization ) , Person ( UnionQueryNamesOnPerson ) } # [ derive ( Serialize ) ] pub struct Variables ; # [ derive ( Deserialize ) ] pub struct ResponseData { pub names : Option < Vec < UnionQueryNames > > , } impl :: graphql_client :: GraphQLQuery for super :: UnionQuery { type Variables = Variables ; type ResponseData = ResponseData ; fn build_query ( variables : Self :: Variables ) -> :: graphql_client :: QueryBody < Self :: Variables > { :: graphql_client :: QueryBody { variables , query : QUERY , operation_name : OPERATION_NAME , } } } } pub mod fragment_on_union { # ! [ allow ( non_camel_case_types ) ] # ! [ allow ( non_snake_case ) ] # ! [ allow ( dead_code ) ] use serde ; pub const OPERATION_NAME : & 'static str = "FragmentOnUnion" ; pub const QUERY : & 'static str = "query UnionQuery {\n  names {\n    __typename\n    ... on Dog {\n      name\n    }\n    ... on Person {\n      firstName\n      lastName\n    }\n    ... on Organization {\n      title\n    }\n  }\n}\n\nfragment NamesFragment on NamedThing {\n  __typename\n  ... on Dog {\n    name\n  }\n  ... on Person {\n    firstName\n  }\n  ... on Organization {\n    title\n  }\n}\n\nquery FragmentOnUnion {\n  names {\n    ...NamesFragment\n  }\n}\n" ; use serde_derive :: * ; # [ allow ( dead_code ) ] type Boolean = bool ; # [ allow ( dead_code ) ] type Float = f64 ; # [ allow ( dead_code ) ] type Int = i64 ; # [ allow ( dead_code ) ] type ID = String ; # [ derive ( Deserialize ) ] pub struct FragmentOnUnionNamesOnDog { pub name : String , } # [ derive ( Deserialize ) ] pub struct FragmentOnUnionNamesOnOrganization { pub title : String , } # [ derive ( Deserialize ) ] pub struct FragmentOnUnionNamesOnPerson { # [ serde ( rename = "firstName" ) ] pub first_name : String , } # [ derive ( Deserialize ) ] # [ serde ( tag = "__typename" ) ] pub enum FragmentOnUnionNames { Dog ( FragmentOnUnionNamesOnDog ) , Organization ( FragmentOnUnionNamesOnOrganization ) , Person ( FragmentOnUnionNamesOnPerson ) } # [ derive ( Serialize ) ] pub struct Variables ; # [ derive ( Deserialize ) ] pub struct ResponseData { pub names : Option < Vec < FragmentOnUnionNames > > , } impl :: graphql_client :: GraphQLQuery for super :: FragmentOnUnion { type Variables = Variables ; type ResponseData = ResponseData ; fn build_query ( variables : Self :: Variables ) -> :: graphql_client :: QueryBody < Self :: Variables > { :: graphql_client :: QueryBody { variables , query : QUERY , operation_name : OPERATION_NAME , } } } }