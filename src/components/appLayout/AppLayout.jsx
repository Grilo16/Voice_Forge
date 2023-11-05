import styled from "styled-components";
import { Header } from "./Header";
import { Footer } from "./Footer";
import { NavBar } from "./NavBar";

export const AppLayout = ({pages, children}) => {
    return (
        <PageContainerDiv>

            <Header/>
            <NavBar pages={pages}/>
            <ContentSection>{children}</ContentSection>
            {/* <Footer/> */}

        </PageContainerDiv>
    )
};

const PageContainerDiv = styled.div`
display: grid;

min-height: 100vh;
max-height: 100vh;
grid-template-rows: 4rem 1fr;
grid-template-columns: 12rem 1fr;
grid-template-areas: 
    "nav header"
    "nav content"
    "nav footer";
`


const ContentSection = styled.section`
background-color: #e7e8e9;
display: flex;
flex-direction: columns;
justify-content: center;
align-items: center ;
min-width: 100%;
`
