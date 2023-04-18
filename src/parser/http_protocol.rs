use std::collections::HashMap;

use rand::Error;
use serde::{Serialize, Deserialize};
use crate::{parser::{split_once_line,  parse_meta_data}, Method, Protocal, Version};

use super::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpParser {
    originContent: String,
}

impl HttpParser {

    pub fn new(str: String) -> HttpParser{
        HttpParser{originContent: str}
    }
    
    pub fn parse(&self) -> Option<HttpRequest> {
        let reuqest_line = self.originContent.split("\r\n");
        
        let http_reqeust:Vec<_>  = reuqest_line.map(|result| result).collect();

        if http_reqeust.len() == 0 {
            return None;
        }
        //println!("reuqest line is {:#?}", http_reqeust);
        let mut cookie= Cookie::new("".to_string());
        let mut uri = "/".to_string();
        let mut url=Path::new(uri);
        let mut method = Method::GET;
        let mut protocol = Protocal::HTTP;
        let mut version = Version::UNKONE;
        let mut others: HashMap<String,String> = HashMap::new();
        let mut line_index = 0;
        let mut body = String::new();

        for line in http_reqeust {

            if line_index == 0 {
                (method,uri, protocol, version) = parse_meta_data(line);
                url = Path::new(uri);

            }else if line.starts_with("{") {
               
                body = line.to_string();
                
                
            }else if line.len() == 0{

            }else{
                let (key, val) = split_once_line(line, ":");
                if key == "Cookie".to_owned() {
                    cookie = Cookie::new(val);

                }else{
                    others.insert(key, val);

                }
            }
            line_index += 1
        }

        let http_header = HttpHeader::new(cookie, url, method, protocol, version, others);
        let http_body  = HttpBody::new(body);
        let http_request = HttpRequest::new(http_header, http_body);
        Some(http_request)

    } 
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequest {
    
    header: HttpHeader,
    body: HttpBody,
}

impl HttpRequest {
    pub fn new(header: HttpHeader, body: HttpBody) ->HttpRequest {
        return HttpRequest{header, body};
    }
    
}



/// Accept	可接受的响应内容类型（Content-Types）。	Accept: text/plain	固定
/// Accept-Charset	可接受的字符集	Accept-Charset: utf-8	固定
/// Accept-Encoding	可接受的响应内容的编码方式。	Accept-Encoding: gzip, deflate	固定
/// Accept-Language	可接受的响应内容语言列表。	Accept-Language: en-US	固定
/// Accept-Datetime	可接受的按照时间来表示的响应内容版本	Accept-Datetime: Sat, 26 Dec 2015 17:30:00 GMT	临时
/// Authorization	用于表示HTTP协议中需要认证资源的认证信息	Authorization: Basic OSdjJGRpbjpvcGVuIANlc2SdDE==	固定
/// Cache-Control	用来指定当前的请求/回复中的，是否使用缓存机制。	Cache-Control: no-cache	固定
/// Connection	客户端（浏览器）想要优先使用的连接类型	Connection: keep-alive
/// Connection: Upgrade 固定
/// Cookie	由之前服务器通过Set-Cookie（见下文）设置的一个HTTP协议Cookie	Cookie: $Version=1; Skin=new;	固定：标准
/// Content-Length	以8进制表示的请求体的长度	Content-Length: 348	固定
/// Content-MD5	请求体的内容的二进制 MD5 散列值（数字签名），以 Base64 编码的结果	Content-MD5: oD8dH2sgSW50ZWdyaIEd9D==	废弃
/// Content-Type	请求体的MIME类型 （用于POST和PUT请求中）	Content-Type: application/x-www-form-urlencoded	固定
/// Date	发送该消息的日期和时间（以RFC 7231中定义的"HTTP日期"格式来发送）	Date: Dec, 26 Dec 2015 17:30:00 GMT	固定
/// Expect	表示客户端要求服务器做出特定的行为	Expect: 100-continue	固定
/// From	发起此请求的用户的邮件地址	From: user@itbilu.com	固定
/// Host	表示服务器的域名以及服务器所监听的端口号。如果所请求的端口是对应的服务的标准端口（80），则端口号可以省略。	Host: www.itbilu.com:80 固定
/// If-Match	仅当客户端提供的实体与服务器上对应的实体相匹配时，才进行对应的操作。主要用于像 PUT 这样的方法中，仅当从用户上次更新某个资源后，该资源未被修改的情况下，才更新该资源。	If-Match: "9jd00cdj34pss9ejqiw39d82f20d0ikd"	固定
/// If-Modified-Since	允许在对应的资源未被修改的情况下返回304未修改	If-Modified-Since: Dec, 26 Dec 2015 17:30:00 GMT	固定
/// If-None-Match	允许在对应的内容未被修改的情况下返回304未修改（ 304 Not Modified ），参考 超文本传输协议 的实体标记	If-None-Match: "9jd00cdj34pss9ejqiw39d82f20d0ikd"	固定
/// If-Range	如果该实体未被修改过，则向返回所缺少的那一个或多个部分。否则，返回整个新的实体	If-Range: "9jd00cdj34pss9ejqiw39d82f20d0ikd"	固定
/// If-Unmodified-Since	仅当该实体自某个特定时间以来未被修改的情况下，才发送回应。	If-Unmodified-Since: Dec, 26 Dec 2015 17:30:00 GMT	固定
/// Max-Forwards	限制该消息可被代理及网关转发的次数。	Max-Forwards: 10	固定
/// Origin	发起一个针对跨域资源共享的请求（该请求要求服务器在响应中加入一个Access-Control-Allow-Origin的消息头，表示访问控制所允许的来源）。	Origin: http://www.itbilu.com	固定: 标准
/// Pragma	与具体的实现相关，这些字段可能在请求/回应链中的任何时候产生。	Pragma: no-cache	固定
/// Proxy-Authorization	用于向代理进行认证的认证信息。	Proxy-Authorization: Basic IOoDZRgDOi0vcGVuIHNlNidJi2==	固定
/// Range	表示请求某个实体的一部分，字节偏移以0开始。	Range: bytes=500-999	固定
/// Referer	表示浏览器所访问的前一个页面，可以认为是之前访问页面的链接将浏览器带到了当前页面。Referer其实是Referrer这个单词，但RFC制作标准时给拼错了，后来也就将错就错使用Referer了。	Referer: http://itbilu.com/nodejs	固定
/// TE	浏览器预期接受的传输时的编码方式：可使用回应协议头Transfer-Encoding中的值（还可以使用"trailers"表示数据传输时的分块方式）用来表示浏览器希望在最后一个大小为0的块之后还接收到一些额外的字段。	TE: trailers,deflate	固定
/// User-Agent	浏览器的身份标识字符串	User-Agent: Mozilla/……	固定
/// Upgrade	要求服务器升级到一个高版本协议。	Upgrade: HTTP/2.0, SHTTP/1.3, IRC/6.9, RTA/x11	固定
/// Via	告诉服务器，这个请求是由哪些代理发出的。	Via: 1.0 fred, 1.1 itbilu.com.com (Apache/1.1)	固定
/// Warning	一个一般性的警告，表示在实体内容体中可能存在错误。	Warning: 199 Miscellaneous warning	固定
#[derive(Serialize, Deserialize, Debug)]
pub struct HttpHeader {
     cookie: Cookie,
     url: Path,
     method: Method,
     protocol: Protocal,
     version: Version,
     others: HashMap<String,String>
}


impl HttpHeader {
    pub fn new(cookie:Cookie, url: Path, method:Method, protocol:Protocal, version:Version, others: HashMap<String, String>) -> HttpHeader {
        return HttpHeader{ cookie, url, method, protocol, version, others };
    }
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpBody {
    content: String,
}

impl HttpBody {
    pub fn new(content: String) -> HttpBody {
        return HttpBody{content};
    }
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cookie {
    OriginalStr: String,
    cookie:HashMap<String, String>,
}


impl Cookie {
    pub fn new(s: String) -> Cookie {
        let mut cookie:HashMap<String, String> = HashMap::new();
        if s.len()  > 0 {
            let s_lines = s.split(";");
            for line in s_lines {
                let (key, val) = split_once_line(line, "=");
                cookie.insert(key, val);
            }
        }
        return Cookie{OriginalStr: s.to_string(), cookie};

    }

}




