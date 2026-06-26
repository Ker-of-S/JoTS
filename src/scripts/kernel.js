// src/scripts/databaseLogic.js

const CHARSET = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,:;!?()[]{}+-=<>&|%*@#^~_αβγδεζηθικλμνξοπρστυφχψω∫∑√≈≠∞";
const BASE = BigInt(CHARSET.length); // 104n

const PAGE_LENGTH = 800; 

function randomFill(seed) {
    let x = Math.sin(seed++) * 10000;
    return x - Math.floor(x);
}

function textToBigInt(text) {
    let result = 0n;
    for (let i = 0; i < text.length; i++) {
        const charIndex = BigInt(CHARSET.indexOf(text[i]));
        result = (result * BASE) + charIndex;
    }
    return result;
}

function bigIntToText(bigNumber) {
    let text = "";
    let currentNumber = bigNumber;
    
    while (currentNumber > 0n) {
        const remainder = Number(currentNumber % BASE);
        text = CHARSET[remainder] + text; 
        currentNumber = currentNumber / BASE; 
    }
    
    while (text.length < PAGE_LENGTH) {
        text = CHARSET[0] + text;
    }
    return text;
}

function calculateOffset(node, cluster, frag) {
    return BigInt(node) * 100000n + BigInt(cluster) * 1000n + BigInt(frag);
}

export function searchDatabase(query) {
    let cleanQuery = "";
    for (let char of query) {
        if (CHARSET.includes(char)) cleanQuery += char;
    }
    if (cleanQuery.length === 0) return null;

    let fullPage = "";
    let seed = cleanQuery.length;
    
    const insertPos = Math.floor(randomFill(seed) * (PAGE_LENGTH - cleanQuery.length));
    
    for (let i = 0; i < PAGE_LENGTH; i++) {
        if (i === insertPos) {
            fullPage += cleanQuery;
            i += cleanQuery.length - 1;
        } else {
            fullPage += CHARSET[Math.floor(randomFill(seed++) * CHARSET.length)];
        }
    }

    const quantumNumber = textToBigInt(fullPage);
    
    const mockNode = (seed % 4) + 1;
    const mockCluster = (seed % 5) + 1;
    const mockFrag = (seed % 32) + 1;

    const offset = calculateOffset(mockNode, mockCluster, mockFrag);
    const sectorMass = quantumNumber - offset;
    
    const coordinateHex = "0x" + sectorMass.toString(16).toUpperCase();
    const displayContent = fullPage.replace(cleanQuery, `<mark class="highlight">${cleanQuery}</mark>`);

    return {
        sector: coordinateHex,
        node: mockNode,
        cluster: mockCluster,
        frag: mockFrag,
        content: displayContent
    };
}

export function exploreCoordinates(sector, node, cluster, frag) {
    try {
        let hexString = sector.trim().toUpperCase();
        if (hexString.startsWith("0X")) hexString = hexString.substring(2);

        const sectorMass = BigInt("0x" + hexString);
        
        const offset = calculateOffset(node, cluster, frag);
        const quantumNumber = sectorMass + offset;

        const content = bigIntToText(quantumNumber);

        return {
            content: content
        };
    } catch (e) {
        return {
            content: "CRITICAL FAILURE: INVALID QUANTUM COORDINATE."
        };
    }
}