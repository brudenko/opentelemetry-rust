# NewRelic OpenTelemetry Example

## Usage

1. Create the OpenTelemetry Collector configuration file and insert the NewRelic licese key there

```bash
cp otel-collector-config.example.yaml otel-collector-config.yaml
```

2. Start the OpenTelemetry Collector

```bash
docker compose up
```

3. Create the python virtual environment

```bash
virtualenv --python=3 venv
source venv/bin/activate
```

4. Generate the OpenTelemetry logs

```bash
python logs.py
```

5. Generate the OpenTelemetry traces

```bash
cargo run
```

6. Check the NewRelic Logs UI

Go to [one.newrelic.com](one.newrelic.com) > __Logs__ and run the following query:

```
newrelic.source:"api.logs.otlp"
```

7. Check the NewRelic Traces UI

Go to [one.newrelic.com](one.newrelic.com) > __Traces__ and find your trace.
