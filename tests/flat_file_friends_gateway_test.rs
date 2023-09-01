use birthday_greetings_kata_rust::friends::flat_file_friends_gateway::FlatFileFriendsGateway;
use birthday_greetings_kata_rust::friends::friend_data::FriendData;
use birthday_greetings_kata_rust::friends::friends_gateway::FriendsGateway;
use chrono::NaiveDate;
use std::io::Result;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn get_friends_from_flat_file() -> Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "last_name, first_name, date_of_birth, email")?;
    writeln!(temp_file, "Franchi, Franca, 24/08/1970, franca@franchi.com")?;
    writeln!(temp_file, "Germi, Mario, 11/12/1980, mario@germi.com")?;
    let flat_file_friends_gateway = FlatFileFriendsGateway::new(temp_file.reopen()?);

    let friends = flat_file_friends_gateway.get_friends();

    assert_eq!(
        friends,
        vec![
            FriendData::new(
                "Franca",
                "Franchi",
                NaiveDate::from_ymd_opt(1970, 8, 24).unwrap(),
                "franca@franchi.com"
            ),
            FriendData::new(
                "Mario",
                "Germi",
                NaiveDate::from_ymd_opt(1980, 12, 11).unwrap(),
                "mario@germi.com"
            )
        ]
    );
    Ok(())
}

#[test]
fn get_no_friends_from_empty_flat_file() -> Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "last_name, first_name, date_of_birth, email")?;
    let flat_file_friends_gateway = FlatFileFriendsGateway::new(temp_file.reopen()?);

    let friends = flat_file_friends_gateway.get_friends();

    assert_eq!(friends, Vec::new());
    Ok(())
}
