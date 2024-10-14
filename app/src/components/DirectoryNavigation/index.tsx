import type { Directory } from "~/services/api/modules";
import { Item } from "./components/Item";

interface Props {
  directories: Directory[];
}

export function DirectoryNavigation({ directories }: Props) {
  return (
    <nav className="">
      {directories.map((dir, index) => (
        <Item key={index} directory={dir} depth={0} />
      ))}
    </nav>
  );
}
