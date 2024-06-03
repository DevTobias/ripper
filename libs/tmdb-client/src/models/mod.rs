pub mod search;
pub use search::GenericSearchResponse;
pub use search::MovieSearchResult;
pub use search::TvSeriesSearchResult;

pub mod movie;
pub use movie::Movie;

pub mod tv_series;
pub use tv_series::Episode;
pub use tv_series::TvSeason;
pub use tv_series::TvSeries;
