use crate::workdir::Workdir;

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["barfoo", "foobar"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

#[test]
fn search() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_json() {
    let wrk = Workdir::new("search_json");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"h1":"foobar","h2":"barfoo"},{"h1":"barfoo","h2":"foobar"}]"#;
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_match() {
    let wrk = Workdir::new("search_match");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn search_match_json() {
    let wrk = Workdir::new("search_match_json");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").arg("--json");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"h1":"foobar","h2":"barfoo"},{"h1":"barfoo","h2":"foobar"}]"#;
    assert_eq!(got, expected);
}

#[test]
fn search_match_with_count() {
    let wrk = Workdir::new("search_match");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");
}

#[test]
fn search_match_quick() {
    let wrk = Workdir::new("search_match_quick");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^a").arg("--quick").arg("data.csv");

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "");
}

#[test]
fn search_nomatch() {
    let wrk = Workdir::new("search_nomatch");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("waldo").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_empty() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("xxx").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_empty_no_headers() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("xxx").arg("data.csv");
    cmd.arg("--no-headers");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_ignore_case() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^FoO").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_ignore_case_count() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^FoO").arg("--count").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^Ḟoo").arg("data.csv");
    cmd.arg("--unicode");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode_count() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^Ḟoo").arg("--count").arg("data.csv");
    cmd.arg("--unicode");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode_envvar() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("^Ḟoo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode_envvar_count() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("^Ḟoo").arg("--count").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers() {
    let wrk = Workdir::new("search_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foobar", "barfoo"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers_json() {
    let wrk = Workdir::new("search_no_headers_json");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").arg("--json");
    cmd.arg("--no-headers");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"0":"foobar","1":"barfoo"},{"0":"barfoo","1":"foobar"}]"#;
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers_count() {
    let wrk = Workdir::new("search_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foobar", "barfoo"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select() {
    let wrk = Workdir::new("search_select");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select_count() {
    let wrk = Workdir::new("search_select");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select_no_headers() {
    let wrk = Workdir::new("search_select_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select_no_headers_count() {
    let wrk = Workdir::new("search_select_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match_count() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "2\n";
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match_no_headers() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--invert-match");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match_no_headers_count() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--invert-match");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "flagged"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "flagged"],
        svec!["a", "b", "0"],
        svec!["barfoo", "foobar", "3"],
        svec!["Ḟooƀar", "ḃarḟoo", "0"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_invert_match() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "flagged"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "flagged"],
        svec!["a", "b", "2"],
        svec!["barfoo", "foobar", "0"],
        svec!["Ḟooƀar", "ḃarḟoo", "4"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_invert_match_count() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo")
        .arg("--count")
        .arg("data.csv")
        .args(["--flag", "flagged"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "flagged"],
        svec!["a", "b", "2"],
        svec!["barfoo", "foobar", "0"],
        svec!["Ḟooƀar", "ḃarḟoo", "4"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_preview() {
    let wrk = Workdir::new("search_preview");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("search");
    cmd.arg("Beacon Hill")
        .arg(test_file)
        .args(["--preview-match", "2"]);

    let preview = wrk.output_stderr(&mut cmd);
    let expected_preview = r#"case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source
101004113298,2022-01-01 00:16:00,2022-04-01 00:16:06,2022-01-10 08:42:23,ONTIME,Closed,Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ,SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing,Inspectional Services,Housing,Unsatisfactory Utilities - Electrical  Plumbing,ISD_Housing (INTERNAL),ISD,,,47 W Cedar St  Boston  MA  02114,3,1B,8,A1,Beacon Hill,14,Ward 5,0504,47 W Cedar St,02114,42.3594,-71.07,Constituent Call
101004141354,2022-01-20 08:07:49,2022-01-21 08:30:00,2022-01-20 08:45:03,ONTIME,Closed,Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ,CE Collection,Public Works Department,Street Cleaning,CE Collection,PWDx_District 1B: North End,PWDx,,,21-23 Temple St  Boston  MA  02114,3,1B,1,A1,Beacon Hill,3,Ward 3,0306,21-23 Temple St,02114,42.3606,-71.0638,City Worker App
Previewed 2 matches in 8 initial records in 0 ms.
"#;
    assert_eq!(preview, expected_preview);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "open_dt", "target_dt", "closed_dt", "ontime", "case_status", "closure_reason", "case_title", "subject", "reason", "type", "queue", "department", "submittedphoto", "closedphoto", "location", "fire_district", "pwd_district", "city_council_district", "police_district", "neighborhood", "neighborhood_services_district", "ward", "precinct", "location_street_name", "location_zipcode", "latitude", "longitude", "source"], 
        svec!["101004113298", "2022-01-01 00:16:00", "2022-04-01 00:16:06", "2022-01-10 08:42:23", "ONTIME", "Closed", "Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ", "SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing", "Inspectional Services", "Housing", "Unsatisfactory Utilities - Electrical  Plumbing", "ISD_Housing (INTERNAL)", "ISD", "", "", "47 W Cedar St  Boston  MA  02114", "3", "1B", "8", "A1", "Beacon Hill", "14", "Ward 5", "0504", "47 W Cedar St", "02114", "42.3594", "-71.07", "Constituent Call"], 
        svec!["101004113298", "2022-01-01 00:16:00", "2022-04-01 00:16:06", "2022-01-10 08:42:23", "ONTIME", "Closed", "Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ", "SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing", "Inspectional Services", "Housing", "Unsatisfactory Utilities - Electrical  Plumbing", "ISD_Housing (INTERNAL)", "ISD", "", "", "47 W Cedar St  Boston  MA  02114", "3", "1B", "8", "A1", "Beacon Hill", "14", "Ward 5", "0504", "47 W Cedar St", "02114", "42.3594", "-71.07", "Constituent Call"],
        svec!["101004141354", "2022-01-20 08:07:49", "2022-01-21 08:30:00", "2022-01-20 08:45:03", "ONTIME", "Closed", "Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ", "CE Collection", "Public Works Department", "Street Cleaning", "CE Collection", "PWDx_District 1B: North End", "PWDx", "", "", "21-23 Temple St  Boston  MA  02114", "3", "1B", "1", "A1", "Beacon Hill", "3", "Ward 3", "0306", "21-23 Temple St", "02114", "42.3606", "-71.0638", "City Worker App"], 
        svec!["101004141367", "2022-01-20 08:15:45", "2022-01-21 08:30:00", "2022-01-20 08:45:12", "ONTIME", "Closed", "Case Closed. Closed date : Thu Jan 20 08:45:12 EST 2022 Noted ", "CE Collection", "Public Works Department", "Street Cleaning", "CE Collection", "PWDx_District 1B: North End", "PWDx", "", "", "12 Derne St  Boston  MA  02114", "3", "1B", "1", "A1", "Beacon Hill", "3", "Ward 3", "0306", "12 Derne St", "02114", "42.3596", "-71.0634", "City Worker App"], 
        svec!["101004113348", "2022-01-01 06:46:29", "2022-01-05 08:30:00", "2022-01-01 15:10:16", "ONTIME", "Closed", "Case Closed. Closed date : Sat Jan 01 15:10:16 EST 2022 Noted Trash bags sent in for collection. No evidence or code violations found at this time  ", "Improper Storage of Trash (Barrels)", "Public Works Department", "Code Enforcement", "Improper Storage of Trash (Barrels)", "PWDx_Code Enforcement", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg", "", "14 S Russell St  Boston  MA  02114", "3", "1B", "1", "A1", "Beacon Hill", "3", "Ward 3", "0306", "14 S Russell St", "02114", "42.3607", "-71.0659", "Citizens Connect App"], 
        svec!["101004113431", "2022-01-01 10:35:45", "2022-01-05 08:30:00", "2022-01-01 14:59:41", "ONTIME", "Closed", "Case Closed. Closed date : Sat Jan 01 14:59:41 EST 2022 Noted Bags sent in for collection. Ticket issued  ", "Improper Storage of Trash (Barrels)", "Public Works Department", "Code Enforcement", "Improper Storage of Trash (Barrels)", "PWDx_Code Enforcement", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d074c005bbcf180c298048/report.jpg", "", "40 Anderson St  Boston  MA  02114", "3", "1B", "8", "A1", "Beacon Hill", "14", "Ward 5", "0504", "40 Anderson St", "02114", "42.3598", "-71.0676", "Citizens Connect App"], 
        svec!["101004113717", "2022-01-01 21:11:00", "2022-01-04 08:30:00", "2022-01-04 09:30:03", "OVERDUE", "Closed", "Case Closed. Closed date : 2022-01-04 09:30:03.91 Case Noted Dear Constituent     NGRID is aware of the broken gate and will send a crew to repair.    We are waiting on there schedule to do so.    Regards   Rich DiMarzo  781-853-9016 ", "Request for Pothole Repair", "Public Works Department", "Highway Maintenance", "Request for Pothole Repair", "PWDx_Contractor Complaints", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d109cf05bbcf180c29c167/Pothole_1.jpg", "", "INTERSECTION of Charles River Plz & Cambridge St  Boston  MA  ", "3", "1B", "7", "A1", "Beacon Hill", "3", "3", "0305", "INTERSECTION Charles River Plz & Cambridge St", "", "42.3594", "-71.0587", "Citizens Connect App"], 
        svec!["101004115066", "2022-01-03 15:51:00", "2022-01-04 15:51:30", "", "OVERDUE", "Open", " ", "Sidewalk Repair (Make Safe)", "Public Works Department", "Highway Maintenance", "Sidewalk Repair (Make Safe)", "PWDx_Highway Construction", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d361c905bbcf180c2b1dd3/report.jpg", "", "64 Anderson St  Boston  MA  02114", "3", "1B", "8", "A1", "Beacon Hill", "14", "Ward 5", "0503", "64 Anderson St", "02114", "42.359", "-71.0676", "Citizens Connect App"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_preview_json() {
    let wrk = Workdir::new("search_preview_json");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("search");
    cmd.arg("Beacon Hill")
        .arg(test_file)
        .arg("--json")
        .arg("--quiet")
        .args(["--preview-match", "2"]);

    let preview = wrk.output_stderr(&mut cmd);
    let expected_preview = r#"[{"case_enquiry_id":"101004113298","open_dt":"2022-01-01 00:16:00","target_dt":"2022-04-01 00:16:06","closed_dt":"2022-01-10 08:42:23","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ","case_title":"SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing","subject":"Inspectional Services","reason":"Housing","type":"Unsatisfactory Utilities - Electrical  Plumbing","queue":"ISD_Housing (INTERNAL)","department":"ISD","submittedphoto":null,"closedphoto":null,"location":"47 W Cedar St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0504","location_street_name":"47 W Cedar St","location_zipcode":"02114","latitude":"42.3594","longitude":"-71.07","source":"Constituent Call"},{"case_enquiry_id":"101004141354","open_dt":"2022-01-20 08:07:49","target_dt":"2022-01-21 08:30:00","closed_dt":"2022-01-20 08:45:03","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ","case_title":"CE Collection","subject":"Public Works Department","reason":"Street Cleaning","type":"CE Collection","queue":"PWDx_District 1B: North End","department":"PWDx","submittedphoto":null,"closedphoto":null,"location":"21-23 Temple St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"21-23 Temple St","location_zipcode":"02114","latitude":"42.3606","longitude":"-71.0638","source":"City Worker App"}]"#;
    assert_eq!(preview, expected_preview);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"case_enquiry_id":"101004113298","open_dt":"2022-01-01 00:16:00","target_dt":"2022-04-01 00:16:06","closed_dt":"2022-01-10 08:42:23","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ","case_title":"SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing","subject":"Inspectional Services","reason":"Housing","type":"Unsatisfactory Utilities - Electrical  Plumbing","queue":"ISD_Housing (INTERNAL)","department":"ISD","submittedphoto":null,"closedphoto":null,"location":"47 W Cedar St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0504","location_street_name":"47 W Cedar St","location_zipcode":"02114","latitude":"42.3594","longitude":"-71.07","source":"Constituent Call"},{"case_enquiry_id":"101004113298","open_dt":"2022-01-01 00:16:00","target_dt":"2022-04-01 00:16:06","closed_dt":"2022-01-10 08:42:23","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ","case_title":"SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing","subject":"Inspectional Services","reason":"Housing","type":"Unsatisfactory Utilities - Electrical  Plumbing","queue":"ISD_Housing (INTERNAL)","department":"ISD","submittedphoto":null,"closedphoto":null,"location":"47 W Cedar St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0504","location_street_name":"47 W Cedar St","location_zipcode":"02114","latitude":"42.3594","longitude":"-71.07","source":"Constituent Call"},{"case_enquiry_id":"101004141354","open_dt":"2022-01-20 08:07:49","target_dt":"2022-01-21 08:30:00","closed_dt":"2022-01-20 08:45:03","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ","case_title":"CE Collection","subject":"Public Works Department","reason":"Street Cleaning","type":"CE Collection","queue":"PWDx_District 1B: North End","department":"PWDx","submittedphoto":null,"closedphoto":null,"location":"21-23 Temple St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"21-23 Temple St","location_zipcode":"02114","latitude":"42.3606","longitude":"-71.0638","source":"City Worker App"},{"case_enquiry_id":"101004141367","open_dt":"2022-01-20 08:15:45","target_dt":"2022-01-21 08:30:00","closed_dt":"2022-01-20 08:45:12","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Thu Jan 20 08:45:12 EST 2022 Noted ","case_title":"CE Collection","subject":"Public Works Department","reason":"Street Cleaning","type":"CE Collection","queue":"PWDx_District 1B: North End","department":"PWDx","submittedphoto":null,"closedphoto":null,"location":"12 Derne St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"12 Derne St","location_zipcode":"02114","latitude":"42.3596","longitude":"-71.0634","source":"City Worker App"},{"case_enquiry_id":"101004113348","open_dt":"2022-01-01 06:46:29","target_dt":"2022-01-05 08:30:00","closed_dt":"2022-01-01 15:10:16","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Sat Jan 01 15:10:16 EST 2022 Noted Trash bags sent in for collection. No evidence or code violations found at this time  ","case_title":"Improper Storage of Trash (Barrels)","subject":"Public Works Department","reason":"Code Enforcement","type":"Improper Storage of Trash (Barrels)","queue":"PWDx_Code Enforcement","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg","closedphoto":null,"location":"14 S Russell St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"14 S Russell St","location_zipcode":"02114","latitude":"42.3607","longitude":"-71.0659","source":"Citizens Connect App"},{"case_enquiry_id":"101004113431","open_dt":"2022-01-01 10:35:45","target_dt":"2022-01-05 08:30:00","closed_dt":"2022-01-01 14:59:41","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Sat Jan 01 14:59:41 EST 2022 Noted Bags sent in for collection. Ticket issued  ","case_title":"Improper Storage of Trash (Barrels)","subject":"Public Works Department","reason":"Code Enforcement","type":"Improper Storage of Trash (Barrels)","queue":"PWDx_Code Enforcement","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d074c005bbcf180c298048/report.jpg","closedphoto":null,"location":"40 Anderson St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0504","location_street_name":"40 Anderson St","location_zipcode":"02114","latitude":"42.3598","longitude":"-71.0676","source":"Citizens Connect App"},{"case_enquiry_id":"101004113717","open_dt":"2022-01-01 21:11:00","target_dt":"2022-01-04 08:30:00","closed_dt":"2022-01-04 09:30:03","ontime":"OVERDUE","case_status":"Closed","closure_reason":"Case Closed. Closed date : 2022-01-04 09:30:03.91 Case Noted Dear Constituent     NGRID is aware of the broken gate and will send a crew to repair.    We are waiting on there schedule to do so.    Regards   Rich DiMarzo  781-853-9016 ","case_title":"Request for Pothole Repair","subject":"Public Works Department","reason":"Highway Maintenance","type":"Request for Pothole Repair","queue":"PWDx_Contractor Complaints","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d109cf05bbcf180c29c167/Pothole_1.jpg","closedphoto":null,"location":"INTERSECTION of Charles River Plz & Cambridge St  Boston  MA  ","fire_district":"3","pwd_district":"1B","city_council_district":"7","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"3","precinct":"0305","location_street_name":"INTERSECTION Charles River Plz & Cambridge St","location_zipcode":null,"latitude":"42.3594","longitude":"-71.0587","source":"Citizens Connect App"},{"case_enquiry_id":"101004115066","open_dt":"2022-01-03 15:51:00","target_dt":"2022-01-04 15:51:30","closed_dt":null,"ontime":"OVERDUE","case_status":"Open","closure_reason":" ","case_title":"Sidewalk Repair (Make Safe)","subject":"Public Works Department","reason":"Highway Maintenance","type":"Sidewalk Repair (Make Safe)","queue":"PWDx_Highway Construction","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d361c905bbcf180c2b1dd3/report.jpg","closedphoto":null,"location":"64 Anderson St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0503","location_street_name":"64 Anderson St","location_zipcode":"02114","latitude":"42.359","longitude":"-71.0676","source":"Citizens Connect App"}]"#;
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}
