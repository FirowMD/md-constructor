export interface MarkdownElement {
    element_name: string;
    element_type: string;
    labels: string[];
    values: string[];
    chosens: string[];
    is_collapsed: boolean;
} 

export interface MarkdownConverter {
    raw_text: string;
    elements: MarkdownElement[];
}