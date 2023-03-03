/*
 * Author: Oyelowo Oyedayo
 * Email: Oyelowo Oyedayo
 * */

use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter, Result as FmtResult},
};

use surrealdb::sql::{self, Table, Value};

use crate::{
    db_field::{Binding, BindingsList, DbFilter, Parametric},
    value_type_wrappers::SurrealId,
    DbField, SurrealdbNode,
};

/// Creates a new `Order` instance with the specified database field.
///
/// # Arguments
///
/// * `field` - A reference to a `DbField` instance to be used as the ordering field.
///
/// # Example
///
/// ```
/// use my_crate::{Order, DbField};
///
/// let id_field = DbField::new("id");
/// let order = Order::new(&id_field);
/// ```
pub fn order(field: &DbField) -> Order {
    Order::new(field)
}

/// Represents an ordering field, direction, and options for a database query.
#[derive(Debug, Clone, Copy)]
pub struct Order<'a> {
    field: &'a DbField,
    direction: Option<OrderDirection>,
    option: Option<OrderOption>,
}

impl<'a> Parametric for Order<'a> {
    fn get_bindings(&self) -> BindingsList {
        self.field.get_bindings()
    }
}

impl<'a> Parametric for &[Order<'a>] {
    fn get_bindings(&self) -> BindingsList {
        todo!()
    }
}

impl<'a> Parametric for Vec<Order<'a>> {
    fn get_bindings(&self) -> BindingsList {
        self.into_iter()
            .flat_map(|o| o.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl<'a> Parametric for Orderables<'a> {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Orderables::Order(o) => o.get_bindings(),
            Orderables::OrdersList(ol) => ol.get_bindings(),
        }
    }
}

pub enum Orderables<'a> {
    Order(Order<'a>),
    OrdersList(Vec<Order<'a>>),
}

impl<'a> From<Order<'a>> for Orderables<'a> {
    fn from(value: Order<'a>) -> Self {
        Self::Order(value)
    }
}

impl<'a> From<Vec<Order<'a>>> for Orderables<'a> {
    fn from(value: Vec<Order<'a>>) -> Self {
        Self::OrdersList(value)
    }
}

impl<'a, const N: usize> From<&[Order<'a>; N]> for Orderables<'a> {
    fn from(value: &[Order<'a>; N]) -> Self {
        Self::OrdersList(value.to_vec())
    }
}

impl<'a> From<Orderables<'a>> for Vec<Order<'a>> {
    fn from(value: Orderables<'a>) -> Self {
        match value {
            Orderables::Order(o) => vec![o.into()],
            Orderables::OrdersList(ol) => ol,
        }
    }
}

impl<'a> Order<'a> {
    /// Creates a new `Order` instance with the specified database field.
    ///
    /// # Arguments
    ///
    /// * `field` - A reference to a `DbField` instance to be used as the ordering field.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field);
    /// ```
    pub fn new(field: &'a DbField) -> Self {
        Order {
            field,
            direction: None,
            option: None,
        }
    }

    /// Sets the direction of the ordering to ascending.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderDirection};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field).asc();
    /// assert_eq!(order.direction, Some(OrderDirection::Asc));
    /// ```
    pub fn asc(mut self) -> Self {
        self.direction = Some(OrderDirection::Asc);
        self
    }

    /// Sets the direction of the ordering to descending.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderDirection};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field).desc();
    /// assert_eq!(order.direction, Some(OrderDirection::Desc));
    /// ```
    pub fn desc(mut self) -> Self {
        self.direction = Some(OrderDirection::Desc);
        self
    }

    /// Sets the ordering option to random.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderOption};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field).rand();
    /// assert_eq!(order.option, Some(OrderOption::Rand));
    /// ```
    pub fn rand(mut self) -> Self {
        self.option = Some(OrderOption::Rand);
        self
    }

    /// Sets the ordering option to collate.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderOption};
    ///
    /// let name_field = DbField::new("name");
    /// let order = Order::new(&name_field).collate();
    /// assert_eq!(order.option, Some(OrderOption::Collate));
    /// ```
    pub fn collate(mut self) -> Self {
        self.option = Some(OrderOption::Collate);
        self
    }

    /// Sets the ordering option to sort the values numerically instead of as strings.
    ///
    /// # Example
    ///
    /// ```
    /// use my_cool_database::query::{Order, DbField};
    ///
    /// let field = DbField::new("age", "users");
    /// let order = Order::new(&field).numeric();
    ///
    /// assert_eq!(order.field.name(), "age");
    /// assert_eq!(order.option.unwrap(), OrderOption::Numeric);
    /// ```
    pub fn numeric(mut self) -> Self {
        self.option = Some(OrderOption::Numeric);
        self
    }
}

impl<'a> Display for &Order<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.field,
            self.option.map_or("".into(), |op| op.to_string()),
            self.direction.unwrap_or(OrderDirection::Asc)
        ))
    }
}

#[derive(Debug, Clone, Copy)]
enum OrderDirection {
    Asc,
    Desc,
}

impl Display for OrderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderDirection::Asc => write!(f, "ASC"),
            OrderDirection::Desc => write!(f, "DESC"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OrderOption {
    Rand,
    Collate,
    Numeric,
}
impl Display for OrderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderOption::Rand => write!(f, "RAND()"),
            OrderOption::Collate => write!(f, "COLLATE"),
            OrderOption::Numeric => write!(f, "NUMERIC"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Targettables<'a> {
    Table(Table),
    Tables(Vec<sql::Table>),
    SurrealId(SurrealId),
    SurrealIds(Vec<SurrealId>),
    // Should already be bound
    SubQuery(QueryBuilder<'a>),
}

impl<'a> From<Vec<sql::Table>> for Targettables<'a> {
    fn from(value: Vec<sql::Table>) -> Self {
        Self::Tables(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}
// impl<'a> From<sql::Tables> for Targettables<'a> {
//     fn from(value: sql::Tables) -> Self {
//         Self::Tables(value)
//     }
// }

impl<'a> From<Vec<sql::Thing>> for Targettables<'a> {
    fn from(value: Vec<sql::Thing>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl<'a> From<sql::Thing> for Targettables<'a> {
    fn from(value: sql::Thing) -> Self {
        Self::SurrealId(value.into())
    }
}

impl<'a, const N: usize> From<&[sql::Table; N]> for Targettables<'a> {
    fn from(value: &[sql::Table; N]) -> Self {
        Self::Tables(value.to_vec())
    }
}

impl<'a, const N: usize> From<&[SurrealId; N]> for Targettables<'a> {
    fn from(value: &[SurrealId; N]) -> Self {
        Self::SurrealIds(value.to_vec())
    }
}

impl<'a, const N: usize> From<&[sql::Thing; N]> for Targettables<'a> {
    fn from(value: &[sql::Thing; N]) -> Self {
        Self::SurrealIds(
            value
                .into_iter()
                .map(|t| t.to_owned().into())
                .collect::<Vec<_>>(),
        )
    }
}

impl<'a> From<Vec<SurrealId>> for Targettables<'a> {
    fn from(value: Vec<SurrealId>) -> Self {
        Self::SurrealIds(value)
    }
}

impl<'a> From<SurrealId> for Targettables<'a> {
    fn from(value: SurrealId) -> Self {
        Self::SurrealId(value)
    }
}

impl<'a> From<Table> for Targettables<'a> {
    fn from(value: Table) -> Self {
        Self::Table(value)
    }
}

impl<'a> From<&mut QueryBuilder<'a>> for Targettables<'a> {
    fn from(value: &mut QueryBuilder<'a>) -> Self {
        Self::SubQuery(value.to_owned())
    }
}

impl<'a> From<QueryBuilder<'a>> for Targettables<'a> {
    fn from(value: QueryBuilder<'a>) -> Self {
        Self::SubQuery(value.to_owned())
    }
}

impl<'a> Parametric for Targettables<'a> {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Targettables::Table(table) => {
                let binding = Binding::new(table.to_owned());
                vec![binding]
            }
            Targettables::Tables(tables) => {
                let bindings = tables
                    .to_vec()
                    .into_iter()
                    .map(Binding::new)
                    .collect::<Vec<_>>();
                bindings
            }
            // Should already be bound
            Targettables::SubQuery(query) => query.get_bindings(),
            Targettables::SurrealId(id) => vec![Binding::new(id.to_owned())],

            Targettables::SurrealIds(ids) => {
                let bindings = ids
                    .into_iter()
                    .map(|id| Binding::new(id.to_owned()))
                    .collect::<Vec<_>>();
                bindings
            }
        }
    }
}

#[derive(Clone)]
pub enum Groupables {
    Field(DbField),
    Fields(Vec<DbField>),
}

impl From<DbField> for Groupables {
    fn from(value: DbField) -> Self {
        Self::Field(value.into())
    }
}

impl From<&DbField> for Groupables {
    fn from(value: &DbField) -> Self {
        Self::Field(value.into())
    }
}

impl<'a, const N: usize> From<&[&DbField; N]> for Groupables {
    fn from(value: &[&DbField; N]) -> Self {
        Self::Fields(value.map(Into::into).to_vec())
    }
}

impl<'a, const N: usize> From<&[DbField; N]> for Groupables {
    fn from(value: &[DbField; N]) -> Self {
        Self::Fields(value.to_vec())
    }
}

impl From<Vec<DbField>> for Groupables {
    fn from(value: Vec<DbField>) -> Self {
        Self::Fields(value)
    }
}

impl From<Vec<&DbField>> for Groupables {
    fn from(value: Vec<&DbField>) -> Self {
        Self::Fields(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl Parametric for Groupables {
    fn get_bindings(&self) -> BindingsList {
        // match self {
        // Splittables::Field(field) => vec![Binding::new(s)],
        // Splittables::Fields(fields) => {
        //     let bindings = fields
        //         .into_iter()
        //         .map(|id| Binding::new(id.to_owned()))
        //         .collect::<Vec<_>>();
        //     bindings
        // }
        // }
        vec![]
    }
}

#[derive(Clone)]
pub enum Splittables {
    Split(DbField),
    Splits(Vec<DbField>),
}

impl From<DbField> for Splittables {
    fn from(value: DbField) -> Self {
        Self::Split(value.into())
    }
}

impl From<&DbField> for Splittables {
    fn from(value: &DbField) -> Self {
        Self::Split(value.into())
    }
}

impl<'a, const N: usize> From<&[&DbField; N]> for Splittables {
    fn from(value: &[&DbField; N]) -> Self {
        Self::Splits(value.map(Into::into).to_vec())
    }
}

impl<'a, const N: usize> From<&[DbField; N]> for Splittables {
    fn from(value: &[DbField; N]) -> Self {
        Self::Splits(value.to_vec())
    }
}

impl From<Vec<DbField>> for Splittables {
    fn from(value: Vec<DbField>) -> Self {
        Self::Splits(value)
    }
}

impl From<Vec<&DbField>> for Splittables {
    fn from(value: Vec<&DbField>) -> Self {
        Self::Splits(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl Parametric for Splittables {
    fn get_bindings(&self) -> BindingsList {
        // match self {
        // Splittables::Split(s) => vec![Binding::new(s)],
        // Splittables::Splits(splits) => {
        //     let bindings = splits
        //         .into_iter()
        //         .map(|id| Binding::new(id.to_owned()))
        //         .collect::<Vec<_>>();
        //     bindings
        // }
        // }
        vec![]
    }
}

/// The query builder struct used to construct complex database queries.
#[derive(Debug, Clone)]
pub struct QueryBuilder<'a> {
    // projections: Vec<&'a str>,
    projections: Vec<String>,
    /// The list of target tables for the query.
    // targets: Vec<&'a str>,
    targets: Vec<String>,
    where_: Option<String>,
    // where_: Option<&'a str>,
    // split: Option<Vec<&'a str>>,
    split: Vec<String>,
    // group_by: Option<Vec<&'a str>>,
    group_by: Vec<String>,
    order_by: Vec<Order<'a>>,
    limit: Option<u64>,
    start: Option<u64>,
    // fetch: Option<Vec<&'a str>>,
    fetch: Vec<String>,
    timeout: Option<&'a str>,
    parallel: bool,
    ________params_accumulator: BindingsList,
}

impl<'a> Parametric for QueryBuilder<'a> {
    fn get_bindings(&self) -> BindingsList {
        self.________params_accumulator.to_vec()
    }
}

impl<'a> QueryBuilder<'a> {
    /// Create a new instance of QueryBuilder.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::QueryBuilder;
    ///
    /// let query_builder = QueryBuilder::new();
    /// ```
    pub fn new() -> QueryBuilder<'a> {
        QueryBuilder {
            projections: vec![],
            targets: vec![],
            where_: None,
            split: vec![],
            group_by: vec![],
            order_by: vec![],
            limit: None,
            start: None,
            fetch: vec![],
            timeout: None,
            parallel: false,
            ________params_accumulator: vec![],
        }
    }

    /// Add a wildcard projection to the query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::QueryBuilder;
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.select_all();
    /// ```
    pub fn select_all(&mut self) -> &mut Self {
        self.projections.push("*".to_string());
        self
    }

    /// Add a projection to the query for a single field.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field to project.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{QueryBuilder, DbField};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.select(DbField("my_field".to_string()));
    /// ```
    pub fn select<'field, T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Cow<'field, DbField>>,
    {
        let field: &DbField = &field.into();
        self.projections.push(field.to_string());
        self
    }

    /// Add projections to the query for multiple fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of field names to project.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{QueryBuilder, DbField};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// let fields = &[DbField("field_1".to_string()), DbField("field_2".to_string())];
    /// query_builder.select_many(fields);
    /// ```
    pub fn select_many<'field, T>(&mut self, fields: &[T]) -> &mut Self
    where
        T: Into<Cow<'field, DbField>> + Clone + Display,
    {
        self.projections.extend_from_slice(
            fields
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        self
    }

    /// Specifies the table to select from.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to select from.
    ///
    /// # Example
    ///
    /// ```
    /// use query_builder::{QueryBuilder, DbField};
    ///
    /// let mut builder = QueryBuilder::select();
    /// builder.from("users");
    ///
    /// assert_eq!(builder.to_string(), "SELECT * FROM users");
    /// ```
    pub fn from(&'a mut self, targettables: impl Into<Targettables<'a>>) -> &'a mut Self {
        let targets: Targettables = targettables.into();
        let targets_bindings = targets.get_bindings();

        // When we have either one or many table names or record ids, we want to use placeholders
        // as the targets which would be bound later but for a subquery in from, that must have
        // already been done by the Subquery(in this case, select query) builder itself
        let target_names = match targets {
            Targettables::Table(_)
            | Targettables::Tables(_)
            | Targettables::SurrealId(_)
            | Targettables::SurrealIds(_) => targets_bindings
                .iter()
                .map(|b| b.get_param().to_string())
                .collect::<Vec<_>>(),
            // Subquery must have be built and interpolated, so no need for rebinding
            Targettables::SubQuery(subquery) => vec![format!("({subquery})")],
        };
        self.update_bindings(targets_bindings);
        self.targets.extend(target_names);
        self
    }

    /// Adds a condition to the `WHERE` clause of the SQL query.
    ///
    /// # Arguments
    ///
    /// * `condition` - A reference to a filter condition.
    ///
    /// # Example
    ///
    /// ```
    /// use query_builder::{QueryBuilder, DbField, DbFilter};
    ///
    /// let mut builder = QueryBuilder::select();
    /// let condition = DbFilter::from(("age", ">", 18));
    /// builder.where_(condition);
    ///
    /// assert_eq!(builder.to_string(), "SELECT * WHERE age > 18");
    /// ```
    pub fn where_(&mut self, condition: impl Into<DbFilter> + Parametric + Clone) -> &mut Self {
        self.update_bindings(condition.get_bindings());
        let condition: DbFilter = condition.into();
        self.where_ = Some(condition.to_string());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        // let mut updated_params = vec![];
        // updated_params.extend(self.________params_accumulator.to_vec());
        // updated_params.extend(parametric_value.get_bindings());
        self.________params_accumulator.extend(bindings);
        self
    }

    /// Adds a field or multiple fields to the `SPLIT BY` clause of the SQL query.
    ///
    /// # Arguments
    ///
    /// * `splittables` - The name of the field or array or vector of fields to add to the `SPLIT BY` clause.
    ///
    /// # Example: For single field
    ///
    /// ```
    /// use query_builder::{QueryBuilder, DbField};
    ///
    /// let mut builder = QueryBuilder::select();
    /// let country = DbField::new("country");
    /// builder.split(country);
    ///
    /// assert_eq!(builder.to_string(), "SELECT * SPLIT BY country");
    ///
    /// ```
    ///
    /// # Examples: For multiple fields
    ///
    /// ```
    ///
    /// let age = DbField::new("age");
    /// let gender = DbField::new("gender");
    /// query = query.split(&[age, gender]);
    ///
    /// assert_eq!(query.build(), "SELECT *, age, gender FROM table SPLIT age, gender");
    /// ```
    pub fn split(&mut self, splittables: impl Into<Splittables>) -> &mut Self {
        let fields: Splittables = splittables.into();
        self.update_bindings(fields.get_bindings());

        let fields = match fields {
            Splittables::Split(one_field) => vec![one_field],
            Splittables::Splits(many_fields) => many_fields,
        };

        // self.split
        //     .extend(fields.iter().map(ToString::to_string).collect::<Vec<_>>());
        fields.iter().for_each(|f| {
            self.split.push(f.to_string());
        });
        self
    }

    /// Sets the GROUP BY clause for the query.
    ///
    /// # Arguments
    ///
    /// * `field(s)` - The name or names of the field to group by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.group_by(DbField::new("age"));
    /// ```
    ///
    ///
    /// # Examples: For multiple fields
    ///
    /// ```
    ///
    /// let age = DbField::new("age");
    /// let gender = DbField::new("gender");
    /// query = query.group_by(&[age, gender]);
    ///
    /// assert_eq!(query.build(), "SELECT *, age, gender FROM table GROUP BY age, gender");
    /// ```
    pub fn group_by(&mut self, groupables: impl Into<Groupables>) -> &mut Self {
        let fields: Groupables = groupables.into();
        self.update_bindings(fields.get_bindings());

        let fields = match fields {
            Groupables::Field(one_field) => vec![one_field],
            Groupables::Fields(many_fields) => many_fields,
        };

        // self.split
        //     .extend(fields.iter().map(ToString::to_string).collect::<Vec<_>>());
        fields.iter().for_each(|f| {
            self.group_by.push(f.to_string());
        });
        self
    }

    /// Sets the ORDER BY clause for the query. Multiple values can also be set within same call.
    /// Repeated calls are accumulated
    ///
    /// # Arguments
    ///
    /// * `orderables` - The field and direction to order by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, Order, Direction, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.order_by(Order::new(DbField::new("age"), Direction::Ascending));
    ///
    /// query_builder.order(&[
    ///     Order::new(DbField::new("age"), Direction::Ascending),
    ///     Order::new(DbField::new("name"), Direction::Descending),
    /// ]);
    /// ```
    pub fn order_by(&mut self, orderables: impl Into<Orderables<'a>>) -> &mut Self {
        let orderables: Orderables = orderables.into();
        self.update_bindings(orderables.get_bindings());

        let orders: Vec<Order> = orderables.into();
        self.order_by.extend(orders);
        self
    }

    /// Sets the LIMIT clause for the query.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of rows to return.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::QueryBuilder;
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.limit(10);
    /// ```
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Adds a start offset to the current query.
    ///
    /// # Arguments
    ///
    /// * `start` - An unsigned 64-bit integer representing the starting offset.
    ///
    /// # Example
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .start(50)
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    /// ```
    ///
    /// # Output
    ///
    /// The `start` method returns a mutable reference to the QueryBuilder instance it was called on,
    /// allowing further method chaining.
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .start(50)
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    ///
    /// assert_eq!(query, "SELECT id, name FROM users OFFSET 50");
    /// ```
    pub fn start(&mut self, start: u64) -> &mut Self {
        self.start = Some(start);
        self
    }

    /// Adds a field to the list of fields to fetch in the current query.
    ///
    /// # Arguments
    ///
    /// * `field` - A reference to a field to be fetched in the query.
    ///
    /// # Example
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    /// ```
    ///
    /// # Output
    ///
    /// The `fetch` method returns a mutable reference to the QueryBuilder instance it was called on,
    /// allowing further method chaining.
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    ///
    /// assert_eq!(query, "SELECT id, name FROM users");
    /// ```
    pub fn fetch<'field, T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Cow<'field, DbField>>,
    {
        let field: &DbField = &field.into();
        self.fetch.push(field.to_string());
        self
    }

    /// Adds multiple fields to the list of fields to fetch in the current query.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of references to fields to be fetched in the query.
    ///
    /// # Example
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let fields = ["id", "name"];
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch_many(&fields)
    ///     .from("users")
    ///     .build();
    /// ```
    ///
    /// # Output
    ///
    /// The `fetch_many` method returns a mutable reference to the QueryBuilder instance it was called on,
    /// allowing further method chaining.
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let fields = ["id", "name"];
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch_many(&fields)
    ///     .from("users")
    ///     .build();
    ///
    /// assert_eq!(query, "SELECT id, name FROM users");
    /// ```
    pub fn fetch_many<'field, T>(&mut self, fields: &[T]) -> &mut Self
    where
        T: Into<Cow<'field, DbField>> + Clone + Display,
    {
        self.fetch.extend_from_slice(
            fields
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        self
    }

    /// Sets the timeout duration for the query.
    ///
    /// # Arguments
    ///
    /// * `duration` - a string slice that specifies the timeout duration. It can be expressed in any format that the database driver supports.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.timeout("5s");
    /// ```
    ///
    /// ---
    ///
    /// Indicates that the query should be executed in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.parallel();
    /// ```
    pub fn timeout(&mut self, duration: &'a str) -> &mut Self {
        self.timeout = Some(duration);
        self
    }

    /// Indicates that the query should be executed in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.parallel();
    /// ```
    pub fn parallel(&mut self) -> &mut Self {
        self.parallel = true;
        self
    }
}
/*
 * Syntax from specs:https://surrealdb.com/docs/surrealql/statements/select
 * SELECT @projections
    FROM @targets
    [ WHERE @condition ]
    [ SPLIT [ AT ] @field ... ]
    [ GROUP [ BY ] @field ... ]
    [ ORDER [ BY ]
        @field [
            RAND()
            | COLLATE
            | NUMERIC
        ] [ ASC | DESC ] ...
    ] ]
    [ LIMIT [ BY ] @limit ]
    [ START [ AT ] @start ]
    [ FETCH @field ... ]
    [ TIMEOUT @duration ]
    [ PARALLEL ]
; */
impl<'a> Display for QueryBuilder<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut query = String::new();

        query.push_str("SELECT ");
        query.push_str(&self.projections.join(", "));
        query.push_str(" FROM ");
        query.push_str(&self.targets.join(", "));

        if let Some(condition) = &self.where_ {
            query.push_str(" WHERE ");
            query.push_str(&condition);
        }

        if !self.split.is_empty() {
            query.push_str(" SPLIT ");
            query.push_str(&self.split.join(", "));
        }

        if !self.group_by.is_empty() {
            query.push_str(" GROUP BY ");
            query.push_str(&self.group_by.join(", "));
        }

        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(
                &self
                    .order_by
                    .iter()
                    .map(|o| format!("{o}"))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        }

        if let Some(limit_value) = self.limit {
            query.push_str(" LIMIT ");
            query.push_str(&limit_value.to_string());
        }

        if let Some(start_value) = self.start {
            query.push_str(" START AT ");
            query.push_str(&start_value.to_string());
        }

        if !self.fetch.is_empty() {
            query.push_str(" FETCH ");
            query.push_str(&self.fetch.join(", "));
        }

        if let Some(timeout_value) = self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(&timeout_value.to_string());
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        query.push(';');
        // Idea
        // println!("VOOOOVOOO ",);
        self.________params_accumulator
            .clone()
            .into_iter()
            .map(|x| {
                let yy = (format!("{}", x.get_param()), format!("{}", x.get_value()));
                dbg!(yy)
            })
            .collect::<Vec<_>>();
        write!(f, "{}", query)
    }
}
