
use crate::service::traits::Service;
use crate::service::types::{AddressRange, Indexed};
use crate::session::UnitIdentifier;
use crate::error::Error;
use crate::channel::{Request, ServiceRequest};

use tokio::sync::oneshot;

impl Service for crate::service::services::ReadDiscreteInputs {

    const REQUEST_FUNCTION_CODE: u8 = crate::function::constants::READ_DISCRETE_INPUTS;

    type Request = AddressRange;
    type Response = Vec<Indexed<bool>>;

    fn is_request_valid(request: &Self::Request) -> bool {
        request.is_valid_for_bits()
    }

    fn create_request(unit_id: UnitIdentifier, argument: Self::Request, reply_to: oneshot::Sender<Result<Self::Response, Error>>) -> Request {
        Request::ReadDiscreteInputs(ServiceRequest::new(unit_id, argument, reply_to))
    }
}
