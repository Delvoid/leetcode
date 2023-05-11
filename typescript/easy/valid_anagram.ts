function isAnagram(s: string, t: string): boolean {
  if (s.length !== t.length) return false;

  const sMap = new Map();

  for (let i = 0; i < s.length; i++) {
    if (sMap.has(s[i])) {
      sMap.set(s[i], sMap.get(s[i]) + 1);
    } else {
      sMap.set(s[i], 1);
    }
    if (sMap.has(t[i])) {
      sMap.set(t[i], sMap.get(t[i]) - 1);
    } else {
      sMap.set(t[i], -1);
    }
  }

  for (let [_, value] of sMap) {
    if (value !== 0) {
      return false;
    }
  }

  return true;
}

function isAnagram_v2(s: string, t: string): boolean {
  if (s.length !== t.length) return false;

  const countEachCharacter = new Array(26).fill(0);

  for (let i = 0; i < s.length; ++i) {
    countEachCharacter[s.charCodeAt(i) - 97] += 1;
    countEachCharacter[t.charCodeAt(i) - 97] -= 1;
  }

  return countEachCharacter.every((count) => count === 0);
}

function isAnagram_v1(s: string, t: string): boolean {
  if (s.length !== t.length) {
    return false;
  }
  const sMap = new Map();

  for (let i = 0; i < s.length; i++) {
    if (sMap.has(s[i])) {
      sMap.set(s[i], sMap.get(s[i]) + 1);
    } else {
      sMap.set(s[i], 1);
    }
  }

  for (let i = 0; i < t.length; i++) {
    if (sMap.has(t[i])) {
      sMap.set(t[i], sMap.get(t[i]) - 1);
    } else {
      return false;
    }
  }

  for (let [_, value] of sMap) {
    if (value !== 0) {
      return false;
    }
  }

  return true;
}
