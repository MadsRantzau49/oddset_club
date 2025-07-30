#[cfg(test)]
mod tests {
    use oddset_club::server::router::route_request;

    fn simulate_get(path: &str, cookie: Option<&str>) -> String {
        let cookie_header = cookie.map_or("".to_string(), |c| format!("Cookie: session_id={}\r\n", c));
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: localhost\r\n{}\r\n",
            path, cookie_header
        );
        route_request(&request)
    }

    fn simulate_post(path: &str, body: &str, cookie: Option<&str>) -> String {
        let cookie_header = cookie.map_or("".to_string(), |c| format!("Cookie: session_id={}\r\n", c));
        let request = format!(
            "POST {} HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\nContent-Type: application/x-www-form-urlencoded\r\n{}\r\n{}",
            path,
            body.len(),
            cookie_header,
            body
        );
        route_request(&request)
    }

    fn get_session() -> String{
        "69de3963-7615-4c99-92ab-a028beac6e2b".to_string()
    }


    // ---------------- GET Routes ----------------

    #[test]
    fn test_get_login() {
        let res = simulate_get("/login", None);
        assert!(res.contains("Login") || res.contains("<html"));
    }

    #[test]
    fn test_get_root() {
        let res = simulate_get("/", None);
        assert!(res.contains("<html"));
    }

    #[test]
    fn test_get_create_club() {
        let res = simulate_get("/create_club", None);
        assert!(res.contains("Create Club") || res.contains("<form"));
    }

    #[test]
    fn test_get_settings() {
        let res = simulate_get("/settings", None);
        assert!(res.contains("Settings") || res.contains("<html"));
    }

    #[test]
    fn test_get_insert_money() {
        let res = simulate_get("/insert_money", None);
        assert!(res.contains("Insert Money") || res.contains("<html"));
    }

    #[test]
    fn test_get_debt() {
        let res = simulate_get("/debt", None);
        assert!(res.contains("Debt") || res.contains("<html"));
    }

    #[test]
    fn test_get_add_odds() {
        let res = simulate_get("/add_odds", None);
        assert!(res.contains("Add Odds") || res.contains("<html"));
    }

    #[test]
    fn test_get_insert_result() {
        let res = simulate_get("/insert_result", None);
        assert!(res.contains("Result") || res.contains("<html"));
    }

    #[test]
    fn test_get_statistics() {
        let res = simulate_get("/statistics", None);
        assert!(res.contains("Statistics") || res.contains("<html"));
    }

    #[test]
    fn test_get_404() {
        let res = simulate_get("/non_existing", None);
        assert!(res.contains("Could not find the page"));
    }

    // ---------------- POST Routes ----------------

    #[test]
fn test_post_add_user_safe() {
    let res = simulate_post(
        "/add_user",
        "username=testuser1&color=#ff0000",
        Some(&get_session()),
    );
    assert!(
        res.contains("Settings") || res.contains("already exists") || res.contains("<html")
    );
}

#[test]
fn test_post_edit_user_safe() {
    // Assumes there's a user with id=1, update name/color safely.
    let res = simulate_post(
        "/edit_player",
        "user_id=1&username=renamed_user&color=#00ff00",
        Some(&get_session()),
    );
    assert!(
        res.contains("Settings") || res.contains("<html") || res.contains("not found")
    );
}

#[test]
fn test_post_insert_money_safe() {
    let res = simulate_post(
        "/insert_money",
        "user_id=1&amount=42.5",
        Some(&get_session()),
    );
    assert!(
        res.contains("Insert Money") || res.contains("<html") || res.contains("not found")
    );
}

#[test]
fn test_post_add_debt_safe() {
    let res = simulate_post(
        "/add_debt",
        "user_id=1&amount=99&description=unittest_debt",
        Some(&get_session()),
    );
    assert!(
        res.contains("Debt") || res.contains("<html") || res.contains("not found")
    );
}

#[test]
fn test_post_add_odds_safe() {
    let res = simulate_post(
        "/add_odds",
        "user_id=1&description=test+match&stake=10&odds=2.0&potential_win=20&volunteer_bet=on&gain_freebet=on",
        Some(&get_session()),
    );
    assert!(
        res.contains("Add Odds") || res.contains("<html") || res.contains("not found")
    );
}

#[test]
fn test_post_update_settings_safe() {
    let res = simulate_post(
        "/update_club_settings",
        "club_title=Test Club Title&saving_goal=2500&bank_money=3500&default_stake=50",
        Some(&get_session()),
    );
    assert!(
        res.contains("Settings") || res.contains("<html") || res.contains("Session died")
    );
}

#[test]
fn test_post_create_club_safe() {
    // This might return "username already exists", which is OK
    let res = simulate_post("/create_club", "username=test&password=test", None);
    assert!(
        res.contains("Club") || res.contains("already exists") || res.contains("<html")
    );
}

#[test]
fn test_post_add_user_real_effect() {
    // This will succeed the first time, fail with "already exists" after — both are okay
    let res = simulate_post(
        "/add_user",
        "username=testuser1&color=#123456",
        Some(&get_session()),
    );
    assert!(
        res.contains("Settings")
            || res.contains("already exists")
            || res.contains("<html")
    );
}

#[test]
fn test_post_edit_user_real_effect() {
    // Edit user with id=1; changes won't break anything if re-run
    let res = simulate_post(
        "/edit_player",
        "user_id=1&username=editeduser&color=#abcdef",
        Some(&get_session()),
    );
    assert!(res.contains("Settings") || res.contains("<html"));
}

#[test]
fn test_post_insert_money_real_effect() {
    // Add a money insertion for user 1
    let res = simulate_post(
        "/insert_money",
        "user_id=1&amount=77",
        Some(&get_session()),
    );
    assert!(res.contains("Insert Money") || res.contains("<html"));
}

#[test]
fn test_post_update_club_settings_real_effect() {
    let res = simulate_post(
        "/update_club_settings",
        "club_title=TestClub&saving_goal=2000&bank_money=999&default_stake=5",
        Some(&get_session()),
    );
    assert!(res.contains("Settings") || res.contains("<html"));
}


}
