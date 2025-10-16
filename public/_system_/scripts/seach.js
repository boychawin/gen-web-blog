"use strict";

function setupSearch(inputId, resultsId, clearBtnId = null) {
    const inputElement = document.getElementById(inputId);
    const resultsContainer = document.getElementById(resultsId);

    let searchTimeout;

    inputElement.addEventListener('input', (e) => {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => {
            performSearch(e.target.value.toLowerCase(), resultsContainer);
        }, 300); 
    });

    if (clearBtnId) {
        const clearBtn = document.getElementById(clearBtnId);
        clearBtn.addEventListener('click', () => {
            inputElement.value = '';
            resultsContainer.classList.add('hidden'); 
            clearBtn.classList.add('hidden');
        });

        inputElement.addEventListener('input', () => {
            clearBtn.classList.toggle('hidden', inputElement.value === '');
        });
    }
}

async function performSearch(query, resultsContainer) {
    if (query.length === 0) {
        resultsContainer.classList.add('hidden');
        return;
    }

    try {
        const response = await fetch('/releases.json', {
            method: 'GET',
            timeout: 5000
        });

        if (!response.ok) throw new Error('Network response was not ok');
        const data = await response.json();
        if (!data.posts) throw new Error('No posts found');

        const filteredResults = data.posts.filter(posts =>
            posts.title.toLowerCase().includes(query)
        );

        resultsContainer.innerHTML = '';
        if (filteredResults.length > 0) {
            filteredResults.forEach(result => {
                const resultItem = document.createElement('div');
                resultItem.classList.add('py-2', 'border-b', 'border-gray-200', 'dark:border-gray-700');
                resultItem.innerHTML = `<a href="${result.url}" class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-600">${result.title}</a>`;
                resultsContainer.appendChild(resultItem);
            });
        } else {
            resultsContainer.innerHTML = '<p class="text-gray-500 dark:text-gray-400">ไม่พบผลลัพธ์</p>';
        }

        resultsContainer.classList.remove('hidden');
    } catch (error) {
        resultsContainer.innerHTML = '<p class="text-red-500">เกิดข้อผิดพลาดในการโหลดข้อมูล</p>';
    }
}

window.setupSearch = setupSearch;