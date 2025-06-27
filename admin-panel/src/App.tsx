import React, { useEffect, useState } from 'react';
import { LineChart, Line, XAxis, YAxis, Tooltip, CartesianGrid, ResponsiveContainer } from 'recharts';

interface MetricPoint {
  time: string;
  uptime: number;
  players: number;
  tick: number;
}

const App: React.FC = () => {
  const [data, setData] = useState<MetricPoint[]>([]);
  const maxPoints = 120; // last 2 minutes

  useEffect(() => {
    const source = new EventSource('/api/server/metrics/stream');
    source.onmessage = (e) => {
      try {
        const payload = JSON.parse(e.data);
        setData((prev) => {
          const next = [...prev, {
            time: new Date().toLocaleTimeString(),
            uptime: payload.uptime_secs,
            players: payload.players,
            tick: payload.avg_tick_ms
          }];
          if (next.length > maxPoints) next.shift();
          return next;
        });
      } catch(err) { console.error(err); }
    };
    return () => source.close();
  }, []);

  const [logs, setLogs] = useState<string>('');
  useEffect(() => {
    const logSource = new EventSource('/api/server/logs/stream');
    logSource.onmessage = (e) => {
      setLogs((prev) => prev + e.data + '\n');
    };
    return () => logSource.close();
  }, []);

  return (
    <div style={{ padding: '16px', color: '#eee', background: '#121212', minHeight: '100vh' }}>
      <h1>GameVerse Dashboard</h1>

      <h2>Avg Tick (ms)</h2>
      <ResponsiveContainer width="100%" height={300}>
        <LineChart data={data} margin={{ top: 20, right: 30, left: 0, bottom: 0 }}>
          <CartesianGrid stroke="#444" />
          <XAxis dataKey="time" stroke="#bbb" tick={{ fontSize: 10 }} />
          <YAxis stroke="#bbb" />
          <Tooltip contentStyle={{ background: '#222', border: 'none' }} />
          <Line type="monotone" dataKey="tick" stroke="#82ca9d" dot={false} isAnimationActive={false} />
        </LineChart>
      </ResponsiveContainer>

      <h2>Logs</h2>
      <pre style={{ background: '#000', padding: '8px', height: '300px', overflowY: 'scroll' }}>
        {logs}
      </pre>
    </div>
  );
};

export default App; 