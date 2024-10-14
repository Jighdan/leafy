import type { Metadata } from "next";
import type { PropsWithChildren } from "react";
import "./globals.css";

export const metadata: Metadata = {
  title: "Leafy",
  description: "Leafy Editor",
};

export default async function Layout({ children }: PropsWithChildren) {
  return (
    <html lang="en">
      <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      </head>

      <body>
        <div className="w-dvw h-dvh max-h-dvh overflow-y-hidden px-8 py-4">{children}</div>
      </body>
    </html>
  );
}
