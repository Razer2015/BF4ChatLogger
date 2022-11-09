# Battlefield 4 - Chat Logger

## Work In Progress

![image](https://user-images.githubusercontent.com/10619845/200950517-00a6b076-cabe-497a-9d10-b00386c216e7.png)

## Configurations

### Environment variables

| Variable name            | Required | Default value            | Description                                                                                                                |
| ------------------------ | -------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| CHRONO_TIMEZONE          | No       | Europe/Helsinki          | Possible values: https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html                                                   |
|||||
| RCON_IP                  | Yes      |                          | Battlefield 4 server RCON IP                                                                                               |
| RCON_PORT                | Yes      |                          | Battlefield 4 server RCON Port                                                                                             |
| RCON_PASSWORD            | Yes      |                          | Battlefield 4 server RCON Password                                                                                         |
|||||
| DISCORD_WEBHOOK          | Yes      |                          | Webhook URL you can create from Discord channel integrations page. If not given, the application will crash.               |
|||||
| RUST_LOG                 | No       | info                     | Log level used for logging (`error`, `warn`, `info`, `debug`, `trace`).                                                    |
|||||

### Notes
