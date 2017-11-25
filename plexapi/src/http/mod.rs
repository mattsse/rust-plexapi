use hyper::header::Headers;

pub enum HttpMethod {
    Get,
    Post,
    Delete
}
/// @see https://github.com/Arcanemagus/plex-api/wiki/Plex-Web-API-Overview#request-headers

/// Platform name, eg iOS, MacOSX, Android, LG, etc
header! { (XPlexPlatform, "X-Plex-Platform") => [String] }

/// Operating system version, eg 4.3.1, 10.6.7, 3.2
header! { (XPlexPlatformVersion, "X-Plex-Platform-Version") => [String] }

/// one or more of [player, controller, server]
header! { (XPlexProviders, "X-Plex-Provides") => [String] }

///UUID, serial number, or other number unique per device
header! { (XPlexClientIdentifier, "X-Plex-Client-Identifier") => [String] }

/// Plex application name, eg Laika, Plex Media Server, Media Link
header! { (XPlexProduct, "X-Plex-Product") => [String] }

/// Plex application version number
header! { (XPlexVersion, "X-Plex-Version") => [String] }

/// Device name and model number, eg iPhone3,2, Motorola XOOM™, LG5200TV
header! { (XPlexDevice, "X-Plex-Device") => [String] }

/// Paging Size, eg Plex-Container-Size=1
header! { (XPlexContainerSize, "X-Plex-Container-Size") => [String] }

/// Paging Start, eg X-Plex-Container-Start=0
header! { (XPlexContainerStart, "X-Plex-Container-Start") => [String] }

/// Authentication token
header! { (XPlexToken, "X-Plex-Token") => [String] }
