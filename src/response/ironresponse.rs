use HttpResponse = http::server::response::ResponseWriter;
use http::server::request::Request;
use http::headers::response::HeaderCollection;
use http::status::Status;

use std::io::IoResult;

use super::Response;

pub struct IronResponse<'a, 'b> {
    writer: &'a mut HttpResponse<'b>
}

impl<'a, 'b> IronResponse<'a, 'b> {
    pub fn from_http<'a, 'b>(res: &'a mut HttpResponse<'b>) -> IronResponse<'a, 'b> {
        IronResponse {
            writer: res
        }
    }
}

impl <'a, 'b> Writer for IronResponse<'a, 'b> {
    fn write(&mut self, content: &[u8]) -> IoResult<()> {
        self.writer.write(content)
    }
}

impl<'a, 'b> Response for IronResponse<'a, 'b> {
    #[inline]
    fn headers_mut<'a>(&'a mut self) -> &'a mut Box<HeaderCollection> { &mut self.writer.headers }

    #[inline]
    fn status_mut<'a>(&'a mut self) -> &'a mut Status { &mut self.writer.status }

    #[inline]
    fn request<'a>(&'a self) -> &'a Request { self.writer.request }

    #[inline]
    fn headers<'a>(&'a self) -> &'a HeaderCollection { &*self.writer.headers }

    #[inline]
    fn status<'a>(&'a self) -> &'a Status { &self.writer.status }
}

