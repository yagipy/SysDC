import { useEffect, useState } from "react";

import Box from "@mui/material/Box";
import List from "@mui/material/List";
import ListItemButton from "@mui/material/ListItemButton";
import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import TextSnippetIcon from "@mui/icons-material/TextSnippet";
import CreateNewFolderOutlinedIcon from '@mui/icons-material/CreateNewFolderOutlined';
import NoteAddOutlinedIcon from '@mui/icons-material/NoteAddOutlined';

import MyFileSystem, { MyNode, MyLeaf } from "../filesystem/MyFileSystem";

interface FileExplorerProps {
    style: React.CSSProperties | undefined,
    fs: MyFileSystem,
    onSelect: (path: string) => void
}

const FileExplorer = (props: FileExplorerProps) => {
    const [entries, setEntries] = useState<JSX.Element[]>([]);

    const createFsEntry = (node: MyNode, depth: number): JSX.Element[] => {
        const dirEntries = Array.from(node.nodes).map(([_, node]) => {
            return (<>
                <ListItemButton
                    sx={{
                        margin: 0,
                        padding: "5px 0 5px 0",
                        pl: depth*2,
                    }}
                    onClick={() => props.onSelect(node.name) }
                >
                    <FolderOpenIcon
                        sx={{
                            padding: "0 5px 0 5px"
                        }} 
                    />
                    { node.name.split("/").slice(-1)[0] }
                </ListItemButton>
                <List
                    style={ props.style }
                >
                    {[ ...createFsEntry(node, depth+1) ]}
                </List>
            </>);
        });

        const fileEntries = Array.from(node.leaves).map(([_, node]) => {
            return (<>
                <ListItemButton
                    sx={{
                        margin: 0,
                        padding: "5px 0 5px 0",
                        pl: depth*2,
                    }}
                    onClick={() => props.onSelect(node.name) }
                >
                    <TextSnippetIcon
                        sx={{
                            padding: "0 5px 0 5px"
                        }} 
                    />
                    { node.name.split("/").slice(-1)[0] }
                </ListItemButton>
            </>);
        });

        return dirEntries.concat(fileEntries);
    };

    const createDirectory = () => {
        const path = prompt("新規作成するディレクトリのパスを入力してください");
        if (path !== null && path !== "") {
            props.fs.mkdir(path);
        }
        setEntries(createFsEntry(props.fs.root, 0));
    };

    const createFile = () => {
        const path = prompt("新規作成するファイルのパスを入力してください");
        if (path !== null && path !== "") {
            props.fs.mkfile(path, "");
        }
        setEntries(createFsEntry(props.fs.root, 0));
    };

    useEffect(() => {
        setEntries(createFsEntry(props.fs.root, 0));
    }, [props.fs]);

    return (
        <Box
            style={ props.style } 
        >
            <Stack
                direction="row"
                justifyContent="center"
                spacing={2}
                sx={{
                    padding: "5px"
                }}
            >
                <Button
                    variant="outlined"
                    size="small"
                    onClick={ createDirectory }
                >
                    <CreateNewFolderOutlinedIcon/>
                </Button>
                <Button
                    variant="outlined"
                    size="small"
                    onClick={ createFile }
                >
                    <NoteAddOutlinedIcon/>
                </Button>
            </Stack>
            <Divider/>
            <List>
                {[ ...entries ]}
            </List>
        </Box>
    );
};

export default FileExplorer;
