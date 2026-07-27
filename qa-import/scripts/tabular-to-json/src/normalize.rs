use chrono::NaiveDate;
use std::collections::HashSet;
use url::{Host, Url};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Field {
    Id,
    Title,
    Preconditions,
    Steps,
    TestData,
    Expected,
    Observed,
    Status,
    Severity,
    Priority,
    Platform,
    Environment,
    Date,
    Notes,
    Category,
}

pub fn field_for_header(header: &str) -> Option<Field> {
    let key = key(header);
    match key.as_str() {
        "id" | "bugid" | "issueid" | "ticketid" | "testcaseid" | "caseid" => Some(Field::Id),
        "title" | "summary" | "name" | "bugtitle" | "issuetitle" | "testcase" => Some(Field::Title),
        "precondition" | "preconditions" | "prerequisite" | "prerequisites" => {
            Some(Field::Preconditions)
        }
        "steps"
        | "reprosteps"
        | "reproductionsteps"
        | "stepstoreproduce"
        | "stepstoreproducebug" => Some(Field::Steps),
        "testdata" | "prompt" | "testdataprompt" | "input" => Some(Field::TestData),
        "expected" | "expectedresult" | "expectedbehavior" | "expectedbehaviour" => {
            Some(Field::Expected)
        }
        "actual" | "actualresult" | "observed" | "observedresult" | "actualbehavior"
        | "actualbehaviour" => Some(Field::Observed),
        "status" | "state" | "result" | "testresult" => Some(Field::Status),
        "severity" | "impact" => Some(Field::Severity),
        "priority" => Some(Field::Priority),
        "platform" | "device" | "os" => Some(Field::Platform),
        "environment" | "env" => Some(Field::Environment),
        "date" | "testdate" | "testedon" | "createddate" | "executedon" => Some(Field::Date),
        "note" | "notes" | "comment" | "comments" => Some(Field::Notes),
        "category" | "type" | "bugtype" => Some(Field::Category),
        _ => None,
    }
}

pub fn header_score(values: &[String]) -> usize {
    values
        .iter()
        .filter_map(|value| field_for_header(value))
        .collect::<HashSet<_>>()
        .len()
}

pub fn text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub fn id(value: Option<&str>) -> Option<String> {
    value
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
}

pub fn status(value: Option<&str>) -> Option<String> {
    text(value).map(|value| match key(&value).as_str() {
        "fail" | "failed" => "Fail".to_string(),
        "pass" | "passed" | "success" => "Pass".to_string(),
        "featurerequest" | "featurerequested" => "Feature Requested".to_string(),
        "pending" => "Pending".to_string(),
        "blocked" => "Blocked".to_string(),
        "notrun" | "nottested" => "Not Run".to_string(),
        "inprogress" => "In Progress".to_string(),
        _ => value,
    })
}

pub fn severity(value: Option<&str>) -> Option<String> {
    let value = text(value)?;
    let tokens = tokens(&value);
    if has(&tokens, &["critical", "blocker", "s1", "sev1"]) {
        Some("Critical".to_string())
    } else if has(&tokens, &["high", "major", "s2", "sev2"]) {
        Some("High".to_string())
    } else if has(&tokens, &["medium", "moderate", "s3", "sev3"]) {
        Some("Medium".to_string())
    } else if has(&tokens, &["low", "minor", "trivial", "s4", "sev4"]) {
        Some("Low".to_string())
    } else {
        None
    }
}

pub fn priority(value: Option<&str>) -> Option<String> {
    let value = text(value)?;
    let first = tokens(&value).into_iter().next().unwrap_or_default();
    if first.len() >= 2 && first.starts_with('p') && first[1..].chars().all(|c| c.is_ascii_digit())
    {
        return Some(first.to_ascii_uppercase());
    }
    canonical_word(&value, &["Critical", "High", "Medium", "Low"])
}

pub fn platform(value: Option<&str>) -> Option<String> {
    let value = text(value)?;
    let normalized = match key(&value).as_str() {
        "web" | "browser" => "Web",
        "ios" | "iphone" | "ipad" => "iOS",
        "android" => "Android",
        "api" => "API",
        "desktop" => "Desktop",
        "mobile" => "Mobile",
        _ => return Some(value),
    };
    Some(normalized.to_string())
}

pub fn environment(value: Option<&str>) -> Option<String> {
    let value = text(value)?;
    let normalized = match key(&value).as_str() {
        "prod" | "production" | "live" => "Production",
        "stage" | "staging" => "Staging",
        "qa" | "test" | "testing" => "QA",
        "dev" | "development" => "Development",
        "local" => "Local",
        _ => return Some(value),
    };
    Some(normalized.to_string())
}

pub fn date(value: Option<&str>, excel_date: Option<&str>) -> Option<String> {
    if let Some(value) = excel_date {
        return Some(value.to_string());
    }
    let value = text(value)?;
    for format in ["%Y-%m-%d", "%Y/%m/%d"] {
        if let Ok(date) = NaiveDate::parse_from_str(&value, format) {
            return Some(date.format("%Y-%m-%d").to_string());
        }
    }
    None
}

pub fn category(value: Option<&str>) -> Option<String> {
    let value = text(value)?;
    canonical_word(
        &value,
        &[
            "Functional",
            "Visual",
            "Accessibility",
            "Console",
            "UX",
            "Content",
        ],
    )
}

pub fn split_steps(raw: Option<&str>) -> Vec<String> {
    let Some(raw) = text(raw) else {
        return Vec::new();
    };
    let lines = raw
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let numbered = lines
        .iter()
        .filter(|line| numbered_step(line).is_some())
        .count();
    if numbered < 2 {
        return vec![raw];
    }
    let mut steps: Vec<String> = Vec::new();
    for line in lines {
        if let Some(step) = numbered_step(line) {
            steps.push(step.to_string());
        } else if let Some(previous) = steps.last_mut() {
            previous.push('\n');
            previous.push_str(line);
        }
    }
    steps
}

pub fn evidence_urls(values: impl Iterator<Item = String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    for value in values {
        let mut search_from = 0;
        while let Some((start, wrapper)) = find_scheme_start(&value, search_from) {
            let end = url_candidate_end(&value, start);
            let candidate = strip_external_closing_wrapper(&value[start..end], wrapper);
            if let Some(canonical) = canonical_http_uri(candidate) {
                if seen.insert(canonical.clone()) {
                    result.push(canonical);
                }
            }
            search_from = end.max(start + 1);
        }
    }
    result
}

fn find_scheme_start(value: &str, from: usize) -> Option<(usize, Option<char>)> {
    let mut index = from;
    while index < value.len() {
        if scheme_length_at(value, index).is_some() {
            if let Some(wrapper) = start_boundary_wrapper(value, index) {
                return Some((index, wrapper));
            }
        }
        index += value[index..].chars().next()?.len_utf8();
    }
    None
}

fn start_boundary_wrapper(value: &str, index: usize) -> Option<Option<char>> {
    if index == 0 {
        return Some(None);
    }
    let (previous_index, previous) = value[..index].char_indices().next_back()?;
    if previous.is_whitespace() {
        return Some(None);
    }
    if matching_closer(previous).is_some()
        && (previous_index == 0
            || value[..previous_index]
                .chars()
                .next_back()
                .is_some_and(char::is_whitespace))
    {
        return Some(Some(previous));
    }
    None
}

fn scheme_length_at(value: &str, index: usize) -> Option<usize> {
    let tail = value.get(index..)?;
    if tail
        .get(..8)
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://"))
    {
        Some(8)
    } else if tail
        .get(..7)
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("http://"))
    {
        Some(7)
    } else {
        None
    }
}

fn url_candidate_end(value: &str, start: usize) -> usize {
    let mut index = start + scheme_length_at(value, start).expect("candidate starts with a scheme");
    while index < value.len() {
        let character = value[index..]
            .chars()
            .next()
            .expect("index is within the string");
        if character.is_whitespace() {
            return index;
        }
        index += character.len_utf8();
    }
    value.len()
}

fn strip_external_closing_wrapper(mut candidate: &str, wrapper: Option<char>) -> &str {
    if let Some(closer) = wrapper.and_then(matching_closer) {
        if candidate.ends_with(closer) {
            candidate = &candidate[..candidate.len() - closer.len_utf8()];
        }
    }
    candidate
}

fn matching_closer(opener: char) -> Option<char> {
    match opener {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        '"' => Some('"'),
        '\'' => Some('\''),
        '`' => Some('`'),
        _ => None,
    }
}

fn canonical_http_uri(candidate: &str) -> Option<String> {
    if has_forbidden_raw_uri_input(candidate)
        || candidate.contains('|')
        || !has_valid_percent_escapes(candidate)
    {
        return None;
    }
    let parsed = Url::parse(candidate).ok()?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none_or(|host| host.is_empty())
    {
        return None;
    }
    let canonical = parsed.to_string();
    (is_rfc3986_serialization(&canonical) && has_only_ipv6_host_brackets(&parsed, &canonical))
        .then_some(canonical)
}

fn has_forbidden_raw_uri_input(candidate: &str) -> bool {
    candidate.contains('\\')
        || candidate
            .bytes()
            .any(|byte| byte == b' ' || byte.is_ascii_control())
        || candidate.bytes().filter(|byte| *byte == b'#').count() > 1
}

fn has_valid_percent_escapes(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len()
                || !bytes[index + 1].is_ascii_hexdigit()
                || !bytes[index + 2].is_ascii_hexdigit()
            {
                return false;
            }
            index += 3;
        } else {
            index += 1;
        }
    }
    true
}

fn is_rfc3986_serialization(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'%' {
            if index + 2 >= bytes.len()
                || !bytes[index + 1].is_ascii_hexdigit()
                || !bytes[index + 2].is_ascii_hexdigit()
            {
                return false;
            }
            index += 3;
        } else if byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'.'
                    | b'_'
                    | b'~'
                    | b':'
                    | b'/'
                    | b'?'
                    | b'#'
                    | b'['
                    | b']'
                    | b'@'
                    | b'!'
                    | b'$'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b';'
                    | b'='
            )
        {
            index += 1;
        } else {
            return false;
        }
    }
    true
}

fn has_only_ipv6_host_brackets(parsed: &Url, canonical: &str) -> bool {
    let opening = canonical.bytes().filter(|byte| *byte == b'[').count();
    let closing = canonical.bytes().filter(|byte| *byte == b']').count();
    match parsed.host() {
        Some(Host::Ipv6(_)) => opening == 1 && closing == 1,
        _ => opening == 0 && closing == 0,
    }
}

fn numbered_step(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let digits = trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }
    let rest = &trimmed[digits..];
    let marker = rest.chars().next()?;
    if !matches!(marker, '.' | ')' | ':' | '-') {
        return None;
    }
    let step = rest[marker.len_utf8()..].trim();
    (!step.is_empty()).then_some(step)
}

fn canonical_word(value: &str, known: &[&str]) -> Option<String> {
    known
        .iter()
        .find(|candidate| key(candidate) == key(value))
        .map(|value| (*value).to_string())
}

fn has(tokens: &[String], candidates: &[&str]) -> bool {
    tokens
        .iter()
        .any(|token| candidates.contains(&token.as_str()))
}

fn tokens(value: &str) -> Vec<String> {
    value
        .to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}

fn key(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_taxonomy_values_are_not_guessed() {
        assert_eq!(severity(Some("needs triage")), None);
        assert_eq!(category(Some("Backend")), None);
    }

    #[test]
    fn preserves_punctuation_concatenated_schemes_and_strips_true_wrappers() {
        let urls = evidence_urls(
            ["See (https://en.wikipedia.org/wiki/Function_(mathematics)) https://a.test/x,https://b.test/y;HTTP://C.test/z https://d.test/one,https://e.test/two [https://wrap.test/x]".to_string()].into_iter(),
        );
        assert_eq!(
            urls,
            [
                "https://en.wikipedia.org/wiki/Function_(mathematics)",
                "https://a.test/x,https://b.test/y;HTTP://C.test/z",
                "https://d.test/one,https://e.test/two",
                "https://wrap.test/x",
            ]
        );
    }

    #[test]
    fn rejects_raw_input_that_whatwg_would_rewrite() {
        for candidate in [
            "https://bad.test/a b",
            "https://bad.test/a\\b",
            "https://bad.test/a\0b",
            "https://bad.test/a#one#two",
        ] {
            assert_eq!(canonical_http_uri(candidate), None, "{candidate:?}");
        }
    }
}
