import { API } from "~/services/api";

interface Props {
  params: { slug: string[] };
}

export default async function Page({ params }: Props) {
  const path = params.slug?.join("/");
  const directory = await API.module("directory").get(path);

  switch (directory.item_type) {
    case "markdown":
      return (
        <textarea defaultValue={directory.content} className="w-full h-full" />
      );

    case "media":
      return (
        <div className="h-full w-full grid place-items-center">
          <img
            src={`data:${directory.mime_type};base64,${directory.media_content}`}
            className="w-full h-auto object-contain"
          />
        </div>
      );

    default:
      return (
        <div className="grid place-items-center h-full w-full">Navigate</div>
      );
  }
}
