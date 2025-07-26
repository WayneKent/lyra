use mpd::Song;
pub(crate) use mpd::{Client, Status, client, error::Error};

pub fn connect(host: &str) -> Result<client::Client, Error> {
    let client = Client::connect(host)?;
    Ok(client)
}

pub fn get_playback_status(client: &mut Client) -> Result<Status, Error> {
    client.status()
}

pub fn get_current_song(client: &mut Client) -> Result<Option<Song>, Error> {
    client.currentsong()
}

#[cfg(test)]
mod tests {

    use crate::mpd::{connect, get_current_song, get_playback_status};

    #[test]
    fn test_connect_success() {
        let result = connect("127.0.0.1:6600");
        println!("{result:#?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_status() {
        let mut client = connect("127.0.0.1:6600").unwrap();
        let status = get_playback_status(&mut client);
        println!("{status:#?}");
        assert!(status.is_ok());
    }

    #[test]
    fn test_get_current_song() {
        let mut client = connect("127.0.0.1:6600").unwrap();
        let song = get_current_song(&mut client);
        println!("{song:#?}");
        assert!(song.is_ok());
    }
}
