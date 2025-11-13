"use client";

import { useEffect, useState } from "react";

type Container = {
  id: string;
  name: string;
  status: string;
};

export default function Home() {
  const [containers, setContainers] = useState<Container[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("http://localhost:8080/api/containers")
      .then((res) => res.json())
      .then((data) => setContainers(data))
      .catch(() => setContainers([]))
      .finally(() => setLoading(false));
  }, []);

  return (
    <main className="min-h-screen px-6 py-10">
      <section className="max-w-5xl mx-auto space-y-6">
        <header>
          <p className="text-sm text-slate-400 uppercase tracking-wide">
            Plataforma de contenedores
          </p>
          <h1 className="text-4xl font-semibold text-white">
            Dashboard inicial
          </h1>
          <p className="text-slate-300 mt-2">
            Este panel consume el backend mock (`/api/containers`) y servirá
            como punto de partida para el UI definitivo.
          </p>
        </header>

        <div className="rounded-2xl border border-white/10 bg-white/5 backdrop-blur p-6">
          <h2 className="text-xl font-medium mb-4">Contenedores recientes</h2>
          {loading && <p>Cargando...</p>}
          {!loading && containers.length === 0 && (
            <p className="text-slate-400">Aún no hay contenedores registrados.</p>
          )}
          <ul className="space-y-3">
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
      </section>
    </main>
  );
}
