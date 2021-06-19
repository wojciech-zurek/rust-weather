# rust-weather
HEAVILY INSPIRED standalone weather app from https://github.com/oh-my-fish/plugin-weather.

You can use it without fish shell.

![rust-weather](readme/shot1.png "rust-weather")

### Requirements:

- rust (for self compiling/building)
- OpenWeather api key: https://openweathermap.org/appid

### Supported language:

- pl
- en

### Supported units:

- metric

### Config:

Create config.json file in you home directory ~/.config/rust-weather/config.json

```cat .config/rust-weather/config.json 
───────┬───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: .config/rust-weather/config.json
───────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ {
   2   │   "api_key" : "your api key",
   3   │   "lang" : "en",
   4   │   "units" : "metric",
   5   │   "latitude" : 50.029468,
   6   │   "longitude" : 22.013083 
   7   │   
   8   │ }
───────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────

```

### Clone, build, run (linux)
```
git clone git@github.com:wojciech-zurek/rust-weather.git
cd rust-weather
RUSTFLAGS="-C target-cpu=native" cargo build --release
sudo cp target/release/rust-weather /bin/weather
mkdir -p ~/.config/rust-weather
echo '{
  "api_key" : "you api key",
  "lang" : "en",
  "units" : "metric",
  "latitude" : 50.029468,
  "longitude" : 22.013083 
 }' > ~/.config/rust-weather/config.json
```

### Todo
- MUCH better error handling,
- support other units,
- cache results,
- use args from command line,
- support query by city name (maybe).
