"use client";

import { useCallback, useEffect, useMemo, useState } from "react";

type Container = {
  id: string;
  name: string;
  status: string;
};

const API_BASE =
  process.env.NEXT_PUBLIC_API_BASE ?? "http://127.0.0.1:8080";
const API_KEY = process.env.NEXT_PUBLIC_API_KEY ?? "";

export default function Home() {
  const [containers, setContainers] = useState<Container[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [version, setVersion] = useState("");

  const fetchContainers = useCallback(async () => {
    setLoading(true);
    try {
      const response = await fetch(`${API_BASE}/api/containers`, {
        headers: { "x-api-key": API_KEY },
      });
      if (!response.ok) {
        throw new Error("No se pudo obtener la lista.");
      }
      const data = await response.json();
      setContainers(data);
      setError(null);
    } catch (err) {
      console.error(err);
      setError("No se pudo comunicar con el backend.");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchContainers();
  }, [fetchContainers]);

  useEffect(() => {
    const endpoint = `${API_BASE}/api/events/containers`;
    const es = new EventSource(endpoint);
    es.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (Array.isArray(data)) {
          setContainers(data);
        }
      } catch (err) {
        console.error("SSE parse error", err);
      }
    };
    es.onerror = () => es.close();
    return () => es.close();
  }, []);

  const handleSubmit = useCallback(
    async (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault();
      if (!name.trim()) {
        setError("El nombre es obligatorio");
        return;
      }
      try {
        const response = await fetch(`${API_BASE}/api/containers`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "x-api-key": API_KEY,
          },
          body: JSON.stringify({ name, version }),
        });
        if (!response.ok) {
          throw new Error("No se pudo crear el contenedor");
        }
        setName("");
        setVersion("");
        await fetchContainers();
      } catch (err) {
        console.error(err);
        setError("Error creando el contenedor.");
      }
    },
    [name, version, fetchContainers]
  );

  const hasContainers = useMemo(() => containers.length > 0, [containers]);

  return (
    <main className="min-h-screen px-6 py-10 bg-slate-950 text-slate-50">
      <section className="max-w-6xl mx-auto space-y-8">
        <header>
          <p className="text-sm text-slate-400 uppercase tracking-wide">
            Plataforma de contenedores
          </p>
          <h1 className="text-4xl font-semibold">Panel principal</h1>
          <p className="text-slate-300 mt-2">
            Gestiona contenedores reales conectados al backend en{" "}
            <code>{API_BASE}</code>.
          </p>
        </header>

        <div className="grid gap-6 lg:grid-cols-2">
          <div className="rounded-2xl border border-white/10 bg-white/5 backdrop-blur p-6 space-y-4">
            <h2 className="text-xl font-medium">Crear contenedor</h2>
            <form className="space-y-4" onSubmit={handleSubmit}>
              <label className="block text-sm">
                Nombre
                <input
                  className="w-full mt-1 rounded-lg bg-slate-900/70 border border-slate-700 px-3 py-2"
                  value={name}
                  onChange={(event) => setName(event.target.value)}
                  placeholder="chrome-beta"
                />
              </label>
              <label className="block text-sm">
                Versión (opcional)
                <input
                  className="w-full mt-1 rounded-lg bg-slate-900/70 border border-slate-700 px-3 py-2"
                  value={version}
                  onChange={(event) => setVersion(event.target.value)}
                  placeholder="118.0"
                />
              </label>
              <button
                type="submit"
                className="w-full rounded-lg bg-brand px-4 py-2 font-semibold text-slate-950 transition hover:bg-brand/80"
              >
                Crear contenedor
              </button>
              {error && <p className="text-rose-400 text-sm">{error}</p>}
            </form>
          </div>

          <div className="rounded-2xl border border-white/10 bg-white/5 backdrop-blur p-6">
            <h2 className="text-xl font-medium mb-4">Contenedores recientes</h2>
            {loading && <p>Cargando...</p>}
            {!loading && !hasContainers && (
              <p className="text-slate-400">
                Aún no hay contenedores registrados.
              </p>
            )}
            <ul className="space-y-3 max-h-80 overflow-auto pr-2">
              {containers.map((container) => (
                <li
                  key={container.id}
                  className="flex items-center justify-between rounded-xl border border-white/10 px-4 py-3"
                >
                  <div>
                    <p className="text-lg font-semibold">{container.name}</p>
                    <p className="text-slate-400 text-sm">{container.id}</p>
                  </div>
                  <span className="text-xs uppercase tracking-wide text-brand">
                    {container.status}
                  </span>
                </li>
              ))}
            </ul>
          </div>
        </div>
      </section>
    </main>
  );
}
