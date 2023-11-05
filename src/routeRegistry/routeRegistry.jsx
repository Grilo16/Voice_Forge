import { Route } from "react-router-dom";

export const pages = {};

export const addPage = (element, label, path, folder=false) => {
    const newPage = {
        element : element,
        label : label,
        path : path,
        folder: folder,
    }
    pages[label] = newPage
};

export const makeRoutes = (pages) => {
    return Object.values(pages).map(({element, path}, index) => <Route key={index} element={element} path={path}/>)
}

