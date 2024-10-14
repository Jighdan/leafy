import type { Directory } from "~/services/api/modules";
import { Icon } from "~/components/Icon";
import cx from "classnames";

interface Props extends Pick<Directory, "item_type"> {
  className?: string;
}

export function ItemIcon({ item_type, className }: Props) {
  const styles = cx("h-4 w-auto aspect-square shrink-0", className);

  switch (item_type) {
    case "directory":
      return <Icon.ChevronRight className={styles} />;

    case "markdown":
      return <Icon.FileText className={styles} />;

    case "media":
      return <Icon.Image className={styles} />;

    default:
      return <Icon.QuestionMark className={styles} />;
  }
}
