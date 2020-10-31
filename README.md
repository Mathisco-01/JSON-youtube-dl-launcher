# JSON-youtube-dl-launcher

Launches youtube-dl commands from a JSON file.
Built to be executed by cron to repeatedly archive entire youtube channels.

Make sure you have youtube-dl installed!
```
pip3 install youtube-dl
```

## JSON data structure
Program reads from JSON file named `channels.json` in the **same** directory as the executable.
```
{"fn": "mathis", "link": "https://www.youtube.com/channel/UCgVMmySjPUNHT60iVEpcNkQ"}
{"fn": "folder name", "link": "https://www.link_to_youtube_channel.com/"}
```

## Dependencies
```
json = "*"
rand = "*"
```
