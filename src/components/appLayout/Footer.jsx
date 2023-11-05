import styled from "styled-components";

export const Footer = () => {
    return (
        <FooterDiv>
            <h1>I'm the Footer</h1>
        </FooterDiv>
    )
};

const FooterDiv = styled.div`
grid-area: footer;
background: linear-gradient(90deg, rgba(240,147,113,1) 0%, rgba(229,44,128,1) 100%);
`