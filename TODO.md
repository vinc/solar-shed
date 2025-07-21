# TODO

- [ ] Rename `victron` to `victron-status`
- [ ] Parse output of `AT+QCSQ` to get RSRP, RSRQ, and SINR to measure 4G/LTE signal
- [ ] Parse `/proc/uptime` to get system uptime

## Notes

Output of `victron-status <dev> <key>`:

```json
{
  "date": "2025-07-21T20:03:54.869173916Z",
  "victron": {
    "mode": "off",
    "load": {
      "A": 0.1
    },
    "solar": {
      "W": 0.0
    },
    "battery": {
      "V": 13.0,
      "A": 0.0
    },
    "yield": {
      "Wh": 60.0
    }
  }
}
```

Output of `modem-status <dev> <key>`:

```json
{
  "date": "2025-07-21T20:03:54.869173916Z",
  "modem": {
    "rsrp": xxx,
    "rsrq": xxx,
    "sinr": xxx
  }
```

Output of `system-status <dev> <key>`:

```json
{
  "date": "2025-07-21T20:03:54.869173916Z",
  "system": {
    "uptime": xxx,
  }
}
```
