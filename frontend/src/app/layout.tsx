import "./globals.css";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Contenedor Win32",
  description: "Panel para administrar contenedores de aplicaciones Windows",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="es">
      <body>{children}</body>
    </html>
  );
}
