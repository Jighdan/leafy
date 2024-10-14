export interface Directory {
	path: string;
	children: Directory[];
	file_extension: string;
	item_type: "markdown" | "directory" | "media" | "other";
}

export type DirectoryContent = MarkdownDirectoryContent | MediaDirectoryContent | OtherDirectoryContent;

export interface MarkdownDirectoryContent extends DirectoryContentMetadata {
	mime_type: string;
	file_extension: string;
	item_type: "markdown";
	content: string;
};

export interface MediaDirectoryContent extends DirectoryContentMetadata {
	mime_type: string;
	item_type: "media";
	media_content: string;
}

export interface OtherDirectoryContent extends DirectoryContentMetadata {
	file_extension: null;
	item_type: "directory" | "other";
}

interface DirectoryContentMetadata {
	path: string;
	size: number;
	last_modified: string;
}