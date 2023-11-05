import { Link } from "react-router-dom";
import styled from "styled-components";

export const NavBar = ({pages}) => {
    const pageLinks = Object.values(pages).map(({label, path, folder}, index) =>  
        <Link style={{textDecoration: "none", color: "white"}} to={path} key={index}>
            <NavItemDiv folder={folder} >
                <NavItemLabel folder={folder}>
                    {label}
                </NavItemLabel>
            </NavItemDiv>
        </Link>
        )
    return (
        <NavDiv>
            {pageLinks}
        </NavDiv>
    )
};


const NavItemLabel = styled.p`
margin-left: ${({folder}) => folder ? "2rem" : "3rem" };
font-weight: ${({folder}) => folder ? "bold": null};
`

const NavItemDiv = styled.div`
background: ${({folder}) => folder ? "#777879" : "#676869"};
min-height: 2rem;
min-width: 12rem;
display: flex;
align-items: center;
justify-content: left;
transition: all 0.2s;
&:hover { 
    background: ${({folder}) => folder ? "#575859" : "#474849"};

}
`


const NavDiv = styled.div`
background-color: #373839;
grid-area: nav;
display: flex;
flex-direction: column;
align-items: center;
`