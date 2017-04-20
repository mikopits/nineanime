use std::fmt;

pub enum Query {
    Genre(Genre),
    GenreMode(GenreMode),
    Season(Season),
    Year(Year),
    Quality(Quality),
    Type(Type),
    Status(Status),
    Sort(Sort),
    Language(Language)
}

pub enum Genre {
    Action, Adventure, Cars, Comedy, Dementia, Demons, Drama, Ecchi, Fantasy,
    Game, Harem, Historical, Horror, Josei, Kids, Magic, MartialArts, Mecha,
    Military, Music, Mystery, Parody, Police, Psychological, Romance, Samurai,
    School, SciFi, Seinen, Shoujo, ShoujoAi, Shounen, ShounenAi, SliceOfLife,
    Space, Sports, SuperPower, Supernatural, Thriller, Vampire, Yaoi, Yuri
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = match *self {
            Genre::Action => 1,
            Genre::Adventure => 2,
            Genre::Cars => 3,
            Genre::Comedy => 4,
            Genre::Dementia => 5,
            Genre::Demons => 6,
            Genre::Drama => 7,
            Genre::Ecchi => 8,
            Genre::Fantasy => 9,
            Genre::Game => 10,
            Genre::Harem => 11,
            // No filter for 12. Hmm... I wonder what it was reserved for :^)
            Genre::Historical => 13,
            Genre::Horror => 14,
            Genre::Josei => 15,
            Genre::Kids => 16,
            Genre::Magic => 17,
            Genre::MartialArts => 18,
            Genre::Mecha => 19,
            Genre::Military => 20,
            Genre::Music => 21,
            Genre::Mystery => 22,
            Genre::Parody => 23,
            Genre::Police => 24,
            Genre::Psychological => 25,
            Genre::Romance => 26,
            Genre::Samurai => 27,
            Genre::School => 28,
            Genre::SciFi => 29,
            Genre::Seinen => 30,
            Genre::Shoujo => 31,
            Genre::ShoujoAi => 32,
            Genre::Shounen => 33,
            Genre::ShounenAi => 34,
            Genre::SliceOfLife => 35,
            Genre::Space => 36,
            Genre::Sports => 37,
            Genre::SuperPower => 38,
            Genre::Supernatural => 39,
            Genre::Thriller => 40,
            Genre::Vampire => 41,
            Genre::Yaoi => 42,
            Genre::Yuri => 43
        };
        write!(f, "genre%5B%5D={}", i)
    }
}

pub enum GenreMode { And, Or }

impl fmt::Display for GenreMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            GenreMode::And => "and",
            GenreMode::Or => "or"
        };
        write!(f, "genre_mode={}", s)
    }
}

pub enum Season {
    Fall,
    Summer,
    Spring,
    Winter,
    Unknown
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Season::Fall => "Fall",
            Season::Summer => "Summer",
            Season::Spring => "Spring",
            Season::Winter => "Winter",
            // Unknow is not a typo. This is what 9anime uses.
            Season::Unknown => "Unknow"
        };
        write!(f, "season%5B%5D={}", s)
    }
}

pub enum Year {
    Given(u16),
    Older
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Year::Given(ref i) => i.to_string(),
            Year::Older => "Older".to_string()
        };
        write!(f, "release%5B%5D={}", s)
    }
}

pub enum Quality {
    HD,
    HDRip,
    SD,
    TS,
    Cam
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Quality::HD => "HD",
            Quality::HDRip => "HDRip",
            Quality::SD => "SD",
            Quality::TS => "TS",
            Quality::Cam => "CAM"
        };
        write!(f, "quality%5B%5D={}", s)
    }
}

pub enum Type {
    Movie,
    TVSeries,
    OVA,
    ONA,
    Special
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Type::Movie => "movie",
            Type::TVSeries => "series",
            Type::OVA => "ova",
            Type::ONA => "ona",
            Type::Special => "special"
        };
        write!(f, "type%5B%5D={}", s)
    }
}

pub enum Status {
    All,
    Airing,
    Finished,
    Upcoming,
    NoUpcoming
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Status::All => "",
            Status::Airing => "airing",
            Status::Finished => "finished",
            Status::Upcoming => "upcoming",
            Status::NoUpcoming => "no-upcoming"
        };
        write!(f, "status%5B%5D={}", s)
    }
}

pub enum Sort {
    Default,
    RecentlyUpdated,
    RecentlyAdded,
    Alphabetical,
    MostWatched,
    Scores,
    ReleaseDate
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Sort::Default => "default",
            Sort::RecentlyUpdated => "episode_last_added_at%3Adesc",
            Sort::RecentlyAdded => "post_date%3Adesc",
            Sort::Alphabetical => "title%3Aasc",
            Sort::MostWatched => "views%3Adesc",
            Sort::Scores => "scores%3Adesc",
            Sort::ReleaseDate => "release_date%3Adesc"
        };
        write!(f, "sort={}", s)
    }
}

pub enum Language {
    All,
    Subbed,
    Dubbed
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Language::All => "all",
            Language::Subbed => "subbed",
            Language::Dubbed => "dubbed"
        };
        write!(f, "language={}", s)
    }
}
