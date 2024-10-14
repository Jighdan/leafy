import { Directory } from "~/services/api/modules";
import { ItemIcon } from "./ItemIcon";
import Link from "next/link";
import cx from "classnames";

interface Props {
  directory: Directory;
  depth: number;
}

export function Item({ directory, depth }: Props) {
  const href = `/editor/${directory.path}`;

  return (
    <div className="block">
      <Link
        href={href}
        shallow
        className="flex items-center gap-2 cursor-pointer"
      >
        <ItemIcon
          item_type={directory.item_type}
          className={cx({
            "transition-transform duration-300 rotate-90":
              directory.item_type === "directory",
          })}
        />

        <span className="line-clamp-1 text-sm">
          {directory.path.split("/").pop()}
        </span>
      </Link>

      {directory.item_type === "directory" &&
        directory.children?.length > 0 && (
          <div className="block ml-2 pl-2 border-l border-l-black/50">
            {directory.children.map((child, index) => (
              <Item key={index} directory={child} depth={depth + 1} />
            ))}
          </div>
        )}
    </div>
  );
}
