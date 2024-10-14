"use client";

import { useState, useEffect, useMemo, useRef } from 'react';
import { Directory } from "~/services/api/modules";
import { ItemIcon } from "./ItemIcon";
import Link from "next/link";
import { usePathname } from 'next/navigation';
import cx from "classnames";

interface Props {
  directory: Directory;
  depth: number;
}

export function Item({ directory, depth }: Props) {
  const [isOpen, setIsOpen] = useState(false);
  const pathname = usePathname();
  const href = useMemo(() => `/editor/${directory.path}`, [directory.path]);
  const [height, setHeight] = useState<number | undefined>(0);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (pathname.startsWith(href)) {
      setIsOpen(true);
    }
  }, [pathname, href]);

  useEffect(() => {
    if (!height || !isOpen || !ref.current) return undefined;
    
    const timeout = setTimeout(() => {
      setHeight(undefined);
    }, 300);

    return () => clearTimeout(timeout);
  }, [height, isOpen]);

  useEffect(() => {
    if (isOpen) {
      const height = ref.current?.getBoundingClientRect().height;
      setHeight(height);
    } else {
      setHeight(0);
    }
  }, [isOpen]);

  const toggleOpen = (e: React.MouseEvent) => {
    if (directory.item_type === "directory" && directory.children?.length > 0) {
      e.preventDefault();
      setIsOpen(!isOpen);
    }
  };

  return (
    <div key={directory.path} className="block group">
      <Link
        href={href}
        shallow
        className="flex items-center gap-2 cursor-pointer"
        onClick={toggleOpen}
      >
        <ItemIcon
          item_type={directory.item_type}
          className={cx({
            "transition-transform duration-300": directory.item_type === "directory",
            "rotate-90": isOpen && directory.item_type === "directory",
          })}
        />
        <span className="line-clamp-1 text-sm">
          {directory.path.split("/").pop()}
        </span>
      </Link>

      {directory.item_type === "directory" &&
        directory.children?.length > 0 && (
          <div 
            className="ml-2 pl-2 border-l border-l-black/50 overflow-hidden transition-[height] duration-300 ease-in-out"
            style={{ height: height === undefined ? 'auto' : `${height}px` }}
          >
            <div ref={ref}>
              {directory.children.map((child, index) => (
                <Item key={index} directory={child} depth={depth + 1} />
              ))}
            </div>
          </div>
        )}
    </div>
  );
}