use std::fmt::Display;
use std::str::FromStr;
use warp::hyper::{Request, Body, Response, StatusCode};
use warp::path::FullPath;

use super::content_type::ContentType;
use super::error::ErrorPage;
use super::insult::Insult;
use super::operation::Operation;
use super::rendering::Render;

lazy_static!(
    static ref ROUTES: Vec<Box<dyn Route>> = {
        let raw: Vec<(&str, &str)> = vec![
            ("/anyway/:company/:from", "Who the fuck are you anyway, :company, why are you stirring up so much trouble, and, who pays you?"),
            ("/asshole/:from", "Fuck you, asshole."),
            ("/awesome/:from", "This is Fucking Awesome."),
            ("/back/:name/:from", ":name, back the fuck off."),
            ("/bag/:from", "Eat a bag of fucking dicks."),
            ("/ballmer/:name/:company/:from", "Fucking :name is a fucking pussy. I'm going to fucking bury that guy, I have done it before, and I will do it again. I'm going to fucking kill :company."),
            ("/bday/:name/:from", "Happy Fucking Birthday, :name."),
            ("/because/:from", "Why? Because fuck you, that's why."),
            ("/blackadder/:name/:from", ":name, your head is as empty as a eunuch’s underpants. Fuck off!"),
            ("/bm/:name/:from", "Bravo mike, :name."),
            ("/bucket/:from", "Please choke on a bucket of cocks."),
            ("/bus/:name/:from", "Christ on a bendy-bus, :name, don't be such a fucking faff-arse."),
            ("/bye/:from", "Fuckity bye!"),
            ("/caniuse/:tool/:from", "Can you use :tool? Fuck no!"),
            ("/chainsaw/:name/:from", "Fuck me gently with a chainsaw, :name. Do I look like Mother Teresa?"),
            ("/cocksplat/:name/:from", "Fuck off :name, you worthless cocksplat."),
            ("/cool/:from", "Cool story, bro."),
            ("/cup/:from", "How about a nice cup of shut the fuck up?"),
            ("/dalton/:name/:from", ":name: A fucking problem solving super-hero."),
            ("/deraadt/:name/:from", ":name you are being the usual slimy hypocritical asshole... You may have had value ten years ago, but people will see that you don't anymore."),
            ("/diabetes/:from", "I'd love to stop and chat to you but I'd rather have type 2 diabetes."),
            ("/donut/:name/:from", ":name, go and take a flying fuck at a rolling donut."),
            ("/dosomething/:do/:something/:from", ":do the fucking :something!"),
            ("/equity/:name/:from", "Equity only? Long hours? Zero Pay? Well :name, just sign me right the fuck up."),
            ("/everything/:from", "Fuck everything."),
            ("/everyone/:from", "Everyone can go and fuck off."),
            ("/family/:from", "Fuck you, your whole family, your pets, and your feces."),
            ("/fascinating/:from", "Fascinating story, in what chapter do you shut the fuck up?"),
            ("/field/:name/:from/:reference", "And :name said unto :from, 'Verily, cast thine eyes upon the field in which I grow my fucks', and :from gave witness unto the field, and saw that it was barren."),
            ("/flying/:from", "I don't give a flying fuck."),
            ("/ftfy/:from", "Fuck That, Fuck You"),
            ("/fts/:name/:from", "Fuck that shit, :name."),
            ("/fewer/:name/:from", "Go fuck yourself :name, you'll disappoint fewer people."),
            ("/fyyff/:from", "Fuck you, you fucking fuck."),
            ("/gfy/:name/:from", "Golf foxtrot yankee, :name."),
            ("/greed/:noun/:from", "The point is, ladies and gentleman, that :noun -- for lack of a better word -- is good. :noun is right. :noun works. :noun clarifies, cuts through, and captures the essence of the evolutionary spirit. :noun, in all of its forms -- :noun for life, for money, for love, knowledge -- has marked the upward surge of mankind."),
            ("/horse/:from", "Fuck you and the horse you rode in on."),
            ("/give/:from", "I give zero fucks."),
            ("/ing/:name/:from", "Fucking fuck off, :name."),
            ("/jinglebells/:from", "Fuck you, fuck me, fuck your family. Fuck your father, fuck your mother, fuck you and me."),
            ("/keep/:name/:from", ":name: Fuck off. And when you get there, fuck off from there too. Then fuck off some more. Keep fucking off until you get back here. Then fuck off again."),
            ("/keepcalm/:reaction/:from", "Keep the fuck calm and :reaction!"),
            ("/king/:name/:from", "Oh fuck off, just really fuck off you total dickface. Christ, :name, you are fucking thick."),
            ("/life/:from", "Fuck my life."),
            ("/linus/:name/:from", ":name, there aren't enough swear-words in the English language, so now I'll have to call you perkeleen vittupää just to express my disgust and frustration with this crap."),
            ("/logs/:from", "Check your fucking logs!"),
            ("/immensity/:from", "You can not imagine the immensity of the FUCK I do not give."),
            ("/look/:name/:from", ":name, do I look like I give a fuck?"),
            ("/looking/:from", "Looking for a fuck to give."),
            ("/madison/:name/:from", "What you've just said is one of the most insanely idiotic things I have ever heard, :name. At no point in your rambling, incoherent response were you even close to anything that could be considered a rational thought. Everyone in this room is now dumber for having listened to it. I award you no points :name, and may God have mercy on your soul."),
            ("/maybe/:from", "Maybe. Maybe not. Maybe fuck yourself."),
            ("/me/:from", "Fuck me."),
            ("/mornin/:from", "Happy fuckin' mornin'!"),
            ("/no/:from", "No fucks given."),
            ("/nugget/:name/:from", "Well :name, aren't you a shining example of a rancid fuck-nugget."),
            ("/off/:name/:from", "Fuck off, :name."),
            ("/off-with/:behavior/:from", "Fuck off with :behavior"),
            ("/outside/:name/:from", ":name, why don't you go outside and play hide-and-go-fuck-yourself?"),
            ("/particular/:thing/:from", "Fuck this :thing in particular."),
            ("/pink/:from", "Well, fuck me pink."),
            ("/problem/:name/:from", "What the fuck is your problem :name?"),
            ("/programmer/:from", "Fuck you, I'm a programmer, bitch!"),
            ("/pulp/:language/:from", ":language, motherfucker, do you speak it?"),
            ("/question/:from", "To fuck off, or to fuck off (that is not a question)"),
            ("/ratsarse/:from", "I don't give a rat's arse."),
            ("/retard/:from", "You Fucktard!"),
            ("/ridiculous/:from", "That's fucking ridiculous"),
            ("/rtfm/:from", "Read the fucking manual!"),
            ("/sake/:from", "For Fuck's sake!"),
            ("/shakespeare/:name/:from", ":name, Thou clay-brained guts, thou knotty-pated fool, thou whoreson obscene greasy tallow-catch!"),
            ("/shit/:from", "Fuck this shit!"),
            ("/shutup/:name/:from", ":name, shut the fuck up."),
            ("/single/:from", "Not a single fuck was given."),
            ("/thanks/:from", "Fuck you very much."),
            ("/that/:from", "Fuck that."),
            ("/think/:name/:from", ":name, you think I give a fuck?"),
            ("/thinking/:name/:from", ":name, what the fuck were you actually thinking?"),
            ("/this/:from", "Fuck this."),
            ("/thumbs/:name/:from", "Who has two thumbs and doesn't give a fuck? :name."),
            ("/too/:from", "Thanks, fuck you too."),
            ("/tucker/:from", "Come the fuck in or fuck the fuck off."),
            ("/waste/:name/:from", "I don't waste my fucking time with your bullshit :name!"),
            ("/what/:from", "What the fuck?!"),
            ("/xmas/:name/:from", "Merry Fucking Christmas, :name."),
            ("/yoda/:name/:from", "Fuck off, you must, :name."),
            ("/you/:name/:from", "Fuck you, :name."),
            ("/zayn/:from", "Ask me if I give a motherfuck ?!!"),
            ("/zero/:from", "Zero, that's the number of fucks I give.")
        ];

        let mut r: Vec<Box<dyn Route>> = Vec::new();
        r.push(Box::new(VersionRoute::new("2.0.0".into())));
        r.push(Box::new(OperationsRoute::new()));
        raw.iter().for_each(|(route, message)| r.push(Box::new(InsultRoute::new(route, String::from(*message)))));
        r
    };
);

trait Route: Send + Sync {
    fn get_operation(&self) -> Operation;
    fn resolve(&self, content_type: ContentType, fields: &Vec<String>) -> String;
    fn matches_uri(&self, uri: &str) -> bool;
    fn matches_fields(&self, field_count: usize) -> bool {
        field_count == 0usize
    }
    fn content_type(&self, req: &Request<Body>) -> Result<ContentType, String> {
        Ok(ContentType::from_request(req)?)
    }
}

struct OperationsRoute;

impl OperationsRoute {
    fn new() -> Self {
        OperationsRoute
    }
}

impl Route for OperationsRoute {
    fn get_operation(&self) -> Operation {
        "/operations".parse().unwrap()
    }
    fn resolve(&self, _: ContentType, _: &Vec<String>) -> String {
        serde_json::to_string(
            &ROUTES
                .iter()
                .map(|o| o.get_operation())
                .collect::<Vec<Operation>>()
        ).expect("Serialization error for operations vec")
    }
    fn matches_uri(&self, uri: &str) -> bool {
        uri == "/operations"
    }
    fn content_type(&self, _: &Request<Body>) -> Result<ContentType, String> {
        Ok(ContentType::Json)
    }
}

struct VersionRoute(String);

impl VersionRoute {
    fn new(version: String) -> Self {
        VersionRoute(version)
    }
}

impl Route for VersionRoute {
    fn get_operation(&self) -> Operation {
        "/version".parse().unwrap()
    }
    fn resolve(&self, content_type: ContentType, _: &Vec<String>) -> String {
        Insult::new(format!("Version {}", self.0), "foaas-rs".into()).render(content_type)
    }
    fn matches_uri(&self, uri: &str) -> bool {
        uri == "/version"
    }
}

struct InsultRoute {
    operation: Operation,
    template: String,
}

impl InsultRoute {
    fn new(uri: &str, template: String) -> Self {
        InsultRoute {
            operation: uri.parse().map_err(|e| error!("{}", e)).unwrap(),
            template,
        }
    }
}

impl Route for InsultRoute {
    fn get_operation(&self) -> Operation {
        self.operation.clone()
    }
    fn resolve(&self, content_type: ContentType, params: &Vec<String>) -> String {
        let mut message = self.template.clone();
        let subtitle = params.last().map(|sub| sub.clone()).unwrap_or_else(|| "".into());
        self.operation.fields
            .iter()
            .zip(params.iter())
            .for_each(|(field, value)| {
                message = message.replace(&format!(":{}", &field.field), &value);
            });
        Insult::new(message, subtitle).render(content_type)
    }
    fn matches_uri(&self, uri: &str) -> bool {
        uri.split("/")
            .collect::<Vec<&str>>()
            .iter()
            .skip(1)
            .next()
            .map(|part| *part == self.operation.name)
            .unwrap_or_else(|| false)
    }
    fn matches_fields(&self, field_count: usize) -> bool {
        self.operation.fields.len() == field_count
    }
}

fn get_route(uri: &str) -> Option<&Box<dyn Route>> {
    ROUTES
        .iter()
        .filter(|r| r.matches_uri(uri))
        .next()
}

fn get_params(uri: &str) -> Vec<String> {
    uri.split("/")
        .skip(2)
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
}

pub(crate) fn prepare_response(path: FullPath, content_type: String, res: &mut Response<Body>) {
    match get_route(path.as_str()) {
        Some(route) => {
            let params = get_params(path.as_str());
            if route.matches_fields(params.len()) {
                match ContentType::from_str(&content_type) {
                    Ok(content_type) => {
                        res.headers_mut().append("Content-Type", content_type.to_header_value());
                        *res.body_mut() = route.resolve(content_type, &params).into();
                    },
                    Err(what) => create_error_page(content_type, res, StatusCode::UNSUPPORTED_MEDIA_TYPE, &what),
                }
            } else {
                create_error_page(content_type, res, StatusCode::BAD_REQUEST, &"Invalid params".to_string());
            }
        },
        None => create_error_page(content_type, res, StatusCode::NOT_FOUND, &"Not found".to_string()),
    }   
}

fn create_error_page<T: Display>(content_type: String, res: &mut Response<Body>, code: StatusCode, what: &T) {
    let content_type = ContentType::from_str(&content_type).unwrap_or(ContentType::Json);
    *res.status_mut() = code;
    res.headers_mut().append("Content-Type", content_type.to_header_value());
    *res.body_mut() = ErrorPage::new(code, what).render(content_type).into();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_route_some() {
        assert!(get_route("/version").is_some());
    }
    #[test]
    fn test_get_route_none() {
        assert!(get_route("invalid").is_none());
    }
    #[test]
    fn test_get_operations() {
        let route = OperationsRoute::new();
        assert!(route.resolve(ContentType::Json, &vec![]).contains("\"url\":\"/operations\""));
    }
    #[test]
    fn test_insult_route_matches_uri_success() {
        let route = InsultRoute::new("/pulp/:language/:from", ":language motherfucker, do you speak it?".into());
        assert!(route.matches_uri("/pulp"));
    }
    #[test]
    fn test_insult_route_matches_uri_failure() {
        let route = InsultRoute::new("/pulp/:language/:from", ":language motherfucker, do you speak it?".into());
        assert!(!route.matches_uri("/pulp2"));
    }
    #[test]
    fn test_get_params() {
        let params = get_params("/hello/world");
        assert_eq!(1usize, params.len());
        assert_eq!(Some(&"world".to_string()), params.get(0));
    }
}