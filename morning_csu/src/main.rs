use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use http::Error;
use serde::{Serialize, Deserialize};
use simplelog::*;
use log::{info, error, warn, debug};


#[actix_web::main]
async fn main() -> Result<(),()> {
    let log_file = match std::fs::File::create("morning_csu.log"){
        Ok(file) => file,
        Err(_) => return Err(()),
    };
    simplelog::WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        log_file,
    ).expect("simplelog init error!");
    println!("Hello, world!");
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Code2SessionParams {
    app_id: String,
    secret: String,
    js_code: String,
}
#[derive(Serialize, Deserialize)]
struct Code2SessionReturn{
    session_key:String,
    unionid:String,
    errmsg:String,
    openid:String,
    errcode:String,
}
///
///  登录凭证校验。通过 wx.login 接口获得临时登录凭证 code 后传到开发者服务器调用此接口完成登录流程。
///
/// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html
/// ```bash
///  GET https://api.weixin.qq.com/sns/jscode2session?appid=APPID&secret=SECRET&js_code=JSCODE&grant_type=authorization_code
/// ```
/// ```
/// params
/// {
/// appid           小程序 appId
/// secret          小程序 appSecret
/// jscode          登录时获取的 code
/// grant_type      授权类型，此处只需填写 authorization_code
/// }
/// return
/// {
/// session_key    会话密钥
/// unionid        用户在开放平台的唯一标识符
/// errmsg         错误信息
/// openid         用户唯一标识
/// errcode        错误码
/// }
/// ```
/// 其中需要需要获得的参数只有jscode，其他参数都是固定的。
/// # Example
/// ```rust
/// let params = Code2SessionParams{
///     app_id:String::from(""),
///     secret:String::from(""),
///     js_code:String::from(""),
///     grant_type:String::from("authorization_code")};
/// let resp = match code_to_session(params).await{
///    Ok(resp) => resp,
///   Err(err) => {
///       let info = format!("code2Session error!,{}",err.to_string());
///       error!("{}",info);
///       err
///     }
/// }
/// ```
///
/// # 错误码
/// 40029	code 无效	js_code无效  
///
/// 45011	api minute-quota reach limit  mustslower  retry next minute	API 调用太频繁，请稍候再试
///
/// 40226	code blocked	高风险等级用户，小程序登录拦截 。风险等级详见用户安全解方案
///
/// -1	    system error	系统繁忙，此时请开发者稍候再试
///
async fn code_to_session(js_code:Code2SessionParams)->Result<Code2SessionReturn,String>{
    let (app_id,secret,js_code) = (js_code.app_id,js_code.secret,js_code.js_code);
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",app_id,secret,js_code);
    let resp = match reqwest::get(&url).await{
        Ok(resp) =>{
            match resp.text().await{
                Ok(resp) => resp,
                Err(err) => {
                    let info = format!("code2Session get response to text error!,{}",err.to_string());
                    error!("{}",info);
                    return Err(String::from("code2Session get response to text error!"));
                }
            }
        }
        Err(err) =>{
            let info = format!("code2Session get response error!,{}",err.to_string());
            error!("{}",info);
            return Err(String::from("code2Session get response error!"));
        }
    };
    let resp:Code2SessionReturn = match serde_json::from_str(&resp){
        Ok(resp) => resp,
        Err(err) => {
            println!("code2Session json parse error!,response is {}",resp);
            let info = format!("code2Session json parse error!,{}",err.to_string());
            error!("{}",info);
            return Err(String::from("code2Session json parse error!"));
        }
    };
    Ok(resp)
}