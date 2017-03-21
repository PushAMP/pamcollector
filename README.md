# pamcollector
PushAmp Metric Collector

Input proto
===========

ts: f64
app_name: String
app_layer: String
value: f32

Example 
    
    $ nc -u 127.0.0.1 12345
    {"ts": 1490170982, "value": 0.4332, "app_name": "App1", "app_layer": "DB"}