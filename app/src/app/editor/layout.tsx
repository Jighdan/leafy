import type { PropsWithChildren } from "react";
import { API } from "~/services/api";
import { DirectoryNavigation } from "~/components/DirectoryNavigation";

export default async function Layout({ children }: PropsWithChildren) {
  const directories = await API.module("directory").get();

  return (
    <main className="grid grid-cols-[0.25fr_1fr] gap-4 h-full">
      <DirectoryNavigation directories={directories} />

      <div className="overflow-y-auto">{children}</div>
    </main>
  );
}
