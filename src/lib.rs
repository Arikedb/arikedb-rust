use tonic::{Request, Response};
use tonic::transport::Channel;
use arikedbpbuff::arikedb_rpc_client::ArikedbRpcClient;
use arikedbpbuff::{
    StatusCode,
    CollectionMeta,
    VariableMeta,
    VarDataPoint,
    VariableEvent,
    ListCollectionsRequest,
    CreateCollectionsRequest,
    DeleteCollectionsRequest,
    ListVariablesRequest,
    CreateVariablesRequest,
    DeleteVariablesRequest,
    SetVariablesRequest,
    GetVariablesRequest,
    SubscribeVariablesRequest,
    AuthenticateRequest,
};

mod arikedbpbuff { tonic::include_proto!("arikedbpbuff"); }


#[derive(Debug)]
pub enum Epoch {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub vtype: VariableType,
    pub buffer_size: u32,
}

#[derive(Debug)]
pub struct Collection {
    pub name: String,
}

#[derive(Debug)]
pub struct DataPoint {
    pub name: String,
    pub vtype: VariableType,
    pub timestamp: String,
    pub epoch: Epoch,
    pub value: String,
}


#[derive(Debug, Default)]
pub enum VariableType {
    I8 = 0,
    I16 = 1,
    I32 = 2,
    I64 = 3,
    I128 = 4,
    U8 = 5,
    U16 = 6,
    U32 = 7,
    U64 = 8,
    U128 = 9,
    F32 = 10,
    #[default]
    F64 = 11,
    STR = 12,
    BOOL = 13,
}

#[derive(Debug, Default)]
pub enum Event {
    #[default]
    OnSet = 0,
    OnChange = 1,
    OnRise = 2,
    OnFall = 3,
    OnValueReachVal = 4,
    OnValueEqVal = 5,
    OnValueLeaveVal = 6,
    OnValueDiffVal = 7,
    OnCrossHighLimit = 8,
    OnCrossLowLimit = 9,
    OnOverHighLimit = 10,
    OnUnderLowLimit = 11,
    OnValueReachRange = 12,
    OnValueInRange = 13,
    OnValueLeaveRange = 14,
    OnValueOutRange = 15,
}

#[derive(Debug, Default)]
pub struct VarEvent {
    pub event: Event,
    pub value: String,
    pub low_limit: String,
    pub high_limit: String,
}

pub struct ArikedbClient {
    client: ArikedbRpcClient<Channel>,
    pub token: Option<String>,
}


impl ArikedbClient {

    pub async fn connect(
        host: &str,
        port: u16,
        use_ssl: bool
    ) -> Result<Self, Box<dyn std::error::Error>> {

        let url = if use_ssl { format!("https://{}:{}", host, port) } else { format!("http://{}:{}", host, port) };

        match Channel::from_shared(url) {
            Ok(endpoint) => {
                match endpoint.connect().await {
                    Ok(channel) => {
                        Ok(Self { client: ArikedbRpcClient::new(channel), token: None })
                    },
                    Err(err) => return Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }

    }

    pub async fn list_collections(
        &mut self
    ) -> Result<Vec<Collection>, Box<dyn std::error::Error>> {

        let mut request = Request::new(ListCollectionsRequest {});
        self._insert_meta(&mut request);

        match self.client.list_collections(request).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(ArikedbClient::_from_coll_meta(response.into_inner().collections))
            }
            Err(err) => Err(err.into()),
        }

    }

    pub async fn create_collections(
        &mut self,
        names: &Vec<&str>
    ) -> Result<(), Box<dyn std::error::Error>> {

        let collections = names.iter().map(|n| CollectionMeta { name: n.to_string() }).collect();
        let mut request = Request::new(CreateCollectionsRequest { collections });
        self._insert_meta(&mut request);

        match self.client.create_collections(request).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(())
            },
            Err(err) => Err(err.into()),
        }

    }

    pub async fn delete_collections(
        &mut self,
        names: &Vec<&str>
    ) -> Result<(), Box<dyn std::error::Error>> {

        let mut request = Request::new(DeleteCollectionsRequest { names: names.iter().map(|n| n.to_string()).collect() });
        self._insert_meta(&mut request);

        match self.client.delete_collections(request).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(())
            },
            Err(err) => Err(err.into()),
        }

    }

    pub async fn list_variables(
        &mut self,
        collection: &str
    ) -> Result<Vec<Variable>, Box<dyn std::error::Error>> {

        let mut request = Request::new(ListVariablesRequest { collection: collection.to_string() });
        self._insert_meta(&mut request);

        match self.client.list_variables(request).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(ArikedbClient::_from_var_meta(response.into_inner().variables))
            }
            Err(err) => Err(err.into()),
        }

    }

    pub async fn create_variables(
        &mut self,
        collection: &str,
        variables: Vec<Variable>
    ) -> Result<(), Box<dyn std::error::Error>> {

        let vars_meta = variables.into_iter().map(|v| VariableMeta {
            name: v.name,
            vtype: v.vtype as i32,
            buffer_size: v.buffer_size
        }).collect();

        let mut request = Request::new(CreateVariablesRequest { collection: collection.to_string(), variables: vars_meta });
        self._insert_meta(&mut request);

        match self.client.create_variables(request).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }

    }

    pub async fn delete_variables(
        &mut self,
        collection: &str,
        names: &Vec<&str>
    ) -> Result<(), Box<dyn std::error::Error>> {

        let mut request = Request::new(DeleteVariablesRequest { collection: collection.to_string(), names: names.iter().map(|n| n.to_string()).collect() });
        self._insert_meta(&mut request);

        match self.client.delete_variables(request).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }

    }

    pub async fn set_variables(
        &mut self,
        collection: &str,
        names: Vec<&str>,
        timestamp: u128,
        values: Vec<&str>,
        epoch: Epoch
    ) -> Result<(), Box<dyn std::error::Error>> {

        let mut requset = Request::new(
            SetVariablesRequest {
                collection: collection.to_string(),
                names: names.iter().map(|n| n.to_string()).collect(),
                timestamp: timestamp.to_string(),
                values: values.iter().map(|v| v.to_string()).collect(),
                epoch: epoch as i32
            }
        );
        self._insert_meta(&mut requset);

        match self.client.set_variables(requset).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(())
            },
            Err(err) => Err(err.into()),
        }

    }

    pub async fn get_variables(
        &mut self,
        collection: &str,
        names: Vec<&str>,
        derived_order: u32,
        epoch: Epoch
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {

        let mut request = Request::new(GetVariablesRequest { collection: collection.to_string(), names: names.iter().map(|n| n.to_string()).collect(), derived_order, epoch: epoch as i32 });
        self._insert_meta(&mut request);

        match self.client.get_variables(request).await {
            Ok(response) => {
                self._read_meta(&response);
                Ok(ArikedbClient::_cast_data_points(response.into_inner().points))
            }
            Err(err) => Err(err.into()),
        }

    }

    pub async fn subscribe_variables<F>(
        &mut self,
        collection: &str,
        names: Vec<&str>,
        events: Vec<VarEvent>,
        callback: F
    ) -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error>>
    where
        F: Fn(DataPoint) -> () + Send + 'static
    {

        let mut request = Request::new(SubscribeVariablesRequest { collection: collection.to_string(), names: names.iter().map(|n| n.to_string()).collect(), events: ArikedbClient::_cast_var_events(events) });
        self._insert_meta(&mut request);

        match self.client.subscribe_variables(request).await {
            Ok(response) => {
                self._read_meta(&response);
                let mut stream = response.into_inner();

                let handler = tokio::spawn(async move {
                    while let Ok(Some(point)) = stream.message().await {
                        callback(ArikedbClient::_cast_data_point(point));
                    }
                });

                Ok(handler)
            },
            Err(err) => Err(err.into()),
        }

    }

    pub async fn authenticate(
        &mut self,
        username: &str,
        password: &str
    ) -> Result<(), Box<dyn std::error::Error>> {

        match self.client.authenticate(AuthenticateRequest {
            username: username.to_string(),
            password: password.to_string()
        }).await {
            Ok(response) => {
                let resp = response.into_inner();
                let status = StatusCode::try_from(resp.status);
                match status {
                    Ok(status_code) => {
                        match status_code {
                            StatusCode::Ok => {
                                self.token = Some(resp.token);
                                Ok(())
                            },
                            StatusCode::Unauthorized => Err("Unauthorized".into()),
                            _ => Err("Unknown error".into()),
                        }
                    }
                    Err(_) => Err("Unknown error".into()),
                }
            },
            Err(err) => Err(err.into()),
        }

    }

    fn _insert_meta<T>(&self, req: &mut Request<T>) {
        match self.token.clone() {
            Some(token) => {
                req.metadata_mut().insert("authorization", token.parse().unwrap());
            }
            None => {},
        };
    }

    fn _read_meta<T>(&mut self, response: &Response<T>) {

        let metadata = response.metadata();
        match metadata.get("refresh_token") {
            Some(token) => {
                self.token = Some(token.to_str().unwrap().to_string());
            },
            None => ()
        };

    }

    fn _from_coll_meta(coll_meta: Vec<CollectionMeta>) -> Vec<Collection> {

        coll_meta.into_iter().map(|meta| Collection { name: meta.name }).collect()

    }

    fn _from_var_meta(var_meta: Vec<VariableMeta>) -> Vec<Variable> {    

        var_meta.into_iter().map(|meta| Variable {
            name: meta.name,
            vtype: ArikedbClient::_variabe_type_cast(meta.vtype),
            buffer_size: meta.buffer_size
        }).collect()

    }

    fn _variabe_type_cast(value: i32) -> VariableType {
        match value {
            0 => VariableType::I8,
            1 => VariableType::I16,
            2 => VariableType::I32,
            3 => VariableType::I64,
            4 => VariableType::I128,
            5 => VariableType::U8,
            6 => VariableType::U16,
            7 => VariableType::U32,
            8 => VariableType::U64,
            9 => VariableType::U128,
            10 => VariableType::F32,
            11 => VariableType::F64,
            12 => VariableType::STR,
            13 => VariableType::BOOL,
            _ => VariableType::I8
        }
    }

    fn _epoch_cast(value: i32) -> Epoch {
        match value {
            0 => Epoch::Second,
            1 => Epoch::Millisecond,
            2 => Epoch::Microsecond,
            3 => Epoch::Nanosecond,
            _ => Epoch::Second
        }
    }

    fn _cast_data_point(point: VarDataPoint) -> DataPoint {
        DataPoint {
            name: point.name,
            vtype: ArikedbClient::_variabe_type_cast(point.vtype),
            timestamp: point.timestamp,
            epoch: ArikedbClient::_epoch_cast(point.epoch),
            value: point.value
        }
    }

    fn _cast_data_points(data_point: Vec<VarDataPoint>) -> Vec<DataPoint> {

        data_point.into_iter().map(|point| ArikedbClient::_cast_data_point(point)).collect()

    }

    fn _event_cast(value: i32) -> Event {
        match value {
            0 => Event::OnSet,
            1 => Event::OnChange,
            2 => Event::OnRise,
            3 => Event::OnFall,
            4 => Event::OnValueReachVal,
            5 => Event::OnValueEqVal,
            6 => Event::OnValueLeaveVal,
            7 => Event::OnValueDiffVal,
            8 => Event::OnCrossHighLimit,
            9 => Event::OnCrossLowLimit,
            10 => Event::OnOverHighLimit,
            11 => Event::OnUnderLowLimit,
            12 => Event::OnValueReachRange,
            13 => Event::OnValueInRange,
            14 => Event::OnValueLeaveRange,
            15 => Event::OnValueOutRange,
            _ => Event::OnSet
        }
    }
    
    fn _cast_var_events(var_events: Vec<VarEvent>) -> Vec<VariableEvent> {

        var_events.into_iter().map(|vevent| VariableEvent {
            event: vevent.event as i32,
            value: vevent.value,
            low_limit: vevent.low_limit,
            high_limit: vevent.high_limit
        }).collect()

    }

}
